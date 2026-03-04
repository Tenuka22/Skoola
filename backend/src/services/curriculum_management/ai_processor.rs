use crate::AppState;
use crate::errors::APIError;
use crate::schema::{lesson_materials, ai_processed_notes};
use crate::database::tables::{LessonMaterial, AiProcessedNote};
use actix_web::web;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct StructuredLessonNotes {
    pub topic: String,
    pub summary: String,
    pub key_takeaways: Vec<String>,
    pub suggested_questions: Vec<String>,
    pub complex_terms: Vec<String>,
}

pub async fn process_material_with_ai(
    pool: web::Data<AppState>,
    material_id: String,
) -> Result<AiProcessedNote, APIError> {
    let mut conn = pool.db_pool.get()?;
    let material = lesson_materials::table.find(&material_id).first::<LessonMaterial>(&mut conn)?;

    let api_key = pool.config.gemini_api_key.as_ref().ok_or_else(|| APIError::internal("Gemini API key not configured"))?;

    // In a real scenario, you'd download the file and send it to Gemini.
    // For now, let's simulate the prompt based on the file name/metadata or assume it's a text-extractable file.
    
    let prompt = format!(
        "Analyze this lesson material (File: {}). Extract a structured summary, key takeaways, and suggested review questions for students. Return the output in strict JSON format matching this structure: {{ 'topic': '...', 'summary': '...', 'key_takeaways': ['...'], 'suggested_questions': ['...'], 'complex_terms': ['...'] }}",
        material.file_name
    );

    let client = Client::new();
    let response = client
        .post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key))
        .json(&serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }]
        }))
        .send()
        .await
        .map_err(|e| APIError::internal(&format!("Failed to call Gemini API: {}", e)))?;

    let res_json: serde_json::Value = response.json().await.map_err(|e| APIError::internal(&format!("Failed to parse Gemini response: {}", e)))?;
    
    // Extract the text from Gemini response (simplified)
    let ai_text = res_json["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or("{}");
    
    // Clean up potential markdown blocks from AI output
    let clean_json = ai_text.trim_start_matches("```json").trim_end_matches("```").trim();

    let structured: StructuredLessonNotes = serde_json::from_str(clean_json)
        .map_err(|e| APIError::internal(&format!("AI returned invalid JSON: {}", e)))?;

    let id = Uuid::new_v4().to_string();
    let new_ai_note = AiProcessedNote {
        id: id.clone(),
        material_id: material_id.clone(),
        structured_json: clean_json.to_string(),
        summary: Some(structured.summary),
        key_takeaways: Some(structured.key_takeaways.join("; ")),
        suggested_questions: Some(structured.suggested_questions.join("; ")),
        created_at: Utc::now().naive_utc(),
    };

    conn.transaction::<_, APIError, _>(|conn| {
        diesel::insert_into(ai_processed_notes::table)
            .values(&new_ai_note)
            .execute(conn)?;

        diesel::update(lesson_materials::table.find(&material_id))
            .set(lesson_materials::is_processed_by_ai.eq(true))
            .execute(conn)?;

        Ok(())
    })?;

    Ok(new_ai_note)
}
