use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ExamRegistrationRequest {
    pub student_id: String,
    pub exam_name: String,
    pub notes: Option<String>,
}

