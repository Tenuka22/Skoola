use crate::errors::APIError;
use crate::models::exams::marking_scheme::MarkingSchemePart;
use crate::models::exams::student_marks::{
    BulkCreateStudentMarkRequest, CreateStudentMarkRequest, StudentMark, StudentMarkEntry,
    StudentMarkEntryHistory, StudentMarkEntryInput, StudentMarkEntryResponse, StudentMarkHistory,
    StudentMarkResponse, UpdateStudentMarkRequest,
};
use crate::schema::{
    marking_scheme_parts, student_mark_entries, student_mark_entries_history, student_marks,
    student_marks_history,
};
use crate::schema::{student_class_assignments, students};
use crate::AppState;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use std::collections::HashMap;
use crate::models::ids::{generate_prefixed_id, IdPrefix};

fn compute_totals(
    parts: &HashMap<String, MarkingSchemePart>,
    entries: &[StudentMarkEntryInput],
) -> Result<(f32, f32, Vec<(StudentMarkEntryInput, f32)>), APIError> {
    let mut total_marks = 0.0f32;
    let mut percentage = 0.0f32;
    let mut normalized_entries = Vec::with_capacity(entries.len());

    for entry in entries {
        let part = parts.get(&entry.marking_scheme_part_id).ok_or_else(|| {
            APIError::bad_request(&format!(
                "Invalid marking scheme part id {}",
                entry.marking_scheme_part_id
            ))
        })?;
        if entry.marks_awarded < 0.0 || entry.marks_awarded > part.max_marks {
            return Err(APIError::bad_request(&format!(
                "Marks for part {} must be between 0 and {}",
                entry.marking_scheme_part_id, part.max_marks
            )));
        }
        total_marks += entry.marks_awarded;
        if part.max_marks > 0.0 {
            percentage += (entry.marks_awarded / part.max_marks) * part.weight_ratio.unwrap_or(0.0);
        }
        normalized_entries.push((entry.clone(), part.max_marks));
    }
    Ok((total_marks, percentage, normalized_entries))
}

fn map_entries(
    rows: Vec<StudentMarkEntry>,
) -> HashMap<String, Vec<StudentMarkEntryResponse>> {
    let mut map: HashMap<String, Vec<StudentMarkEntryResponse>> = HashMap::new();
    for row in rows {
        map.entry(row.student_mark_id.clone())
            .or_default()
            .push(StudentMarkEntryResponse {
                id: row.id,
                marking_scheme_part_id: row.marking_scheme_part_id,
                marks_awarded: row.marks_awarded,
                max_marks: row.max_marks,
            });
    }
    map
}

pub async fn create_student_mark(
    pool: web::Data<AppState>,
    req: CreateStudentMarkRequest,
    current_user_id: String,
) -> Result<StudentMarkResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let parts: Vec<MarkingSchemePart> = marking_scheme_parts::table
        .filter(marking_scheme_parts::scheme_id.eq(&req.marking_scheme_id))
        .load(&mut conn)?;
    let part_map: HashMap<String, MarkingSchemePart> =
        parts.into_iter().map(|p| (p.id.clone(), p)).collect();

    let (total_marks, percentage, normalized_entries) =
        compute_totals(&part_map, &req.entries)?;

    let now = Utc::now().naive_utc();
    let mark_id = generate_prefixed_id(&mut conn, IdPrefix::STUDENT_MARK)?;
    let student_mark = StudentMark {
        id: mark_id.clone(),
        student_id: req.student_id,
        subject_id: req.subject_id,
        assessment_type: req.assessment_type,
        assessment_id: req.assessment_id,
        marking_scheme_id: req.marking_scheme_id,
        total_marks: Some(total_marks),
        percentage: Some(percentage),
        grade: None,
        grade_point: None,
        is_absent: req.is_absent.unwrap_or(false),
        remarks: req.remarks,
        entered_by: current_user_id.clone(),
        entered_at: now,
        updated_by: None,
        updated_at: now,
    };

    let mut entries_rows = Vec::new();
    for (e, max_marks) in normalized_entries {
        entries_rows.push(StudentMarkEntry {
            id: generate_prefixed_id(&mut conn, IdPrefix::STUDENT_MARK)?,
            student_mark_id: mark_id.clone(),
            marking_scheme_part_id: e.marking_scheme_part_id,
            marks_awarded: e.marks_awarded,
            max_marks,
            created_at: now,
            updated_at: now,
        });
    }

    conn.transaction::<_, APIError, _>(|conn| {
        diesel::insert_into(student_marks::table)
            .values(&student_mark)
            .execute(conn)?;
        if !entries_rows.is_empty() {
            diesel::insert_into(student_mark_entries::table)
                .values(&entries_rows)
                .execute(conn)?;
        }
        Ok(())
    })?;

    let entries = entries_rows
        .into_iter()
        .map(|e| StudentMarkEntryResponse {
            id: e.id,
            marking_scheme_part_id: e.marking_scheme_part_id,
            marks_awarded: e.marks_awarded,
            max_marks: e.max_marks,
        })
        .collect();
    Ok(StudentMarkResponse::from_with_entries(student_mark, entries))
}

pub async fn get_student_mark_by_id(
    pool: web::Data<AppState>,
    student_mark_id: String,
) -> Result<StudentMarkResponse, APIError> {
    let mut conn = pool.db_pool.get()?;
    let student_mark: StudentMark = student_marks::table
        .filter(student_marks::id.eq(&student_mark_id))
        .first(&mut conn)?;
    let entry_rows: Vec<StudentMarkEntry> = student_mark_entries::table
        .filter(student_mark_entries::student_mark_id.eq(&student_mark_id))
        .load(&mut conn)?;
    let entries = map_entries(entry_rows)
        .remove(&student_mark_id)
        .unwrap_or_default();
    Ok(StudentMarkResponse::from_with_entries(student_mark, entries))
}

pub async fn get_all_student_marks(
    pool: web::Data<AppState>,
    last_id: Option<String>,
    limit: Option<i64>,
) -> Result<Vec<StudentMarkResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut query = student_marks::table.into_boxed();
    if let Some(last_id) = &last_id {
        query = query.filter(student_marks::id.gt(last_id));
    }
    let limit = limit.unwrap_or(10);
    let marks: Vec<StudentMark> = query
        .order(student_marks::entered_at.desc())
        .limit(limit)
        .load::<StudentMark>(&mut conn)?;

    let ids: Vec<String> = marks.iter().map(|m| m.id.clone()).collect();
    let entry_rows: Vec<StudentMarkEntry> = if ids.is_empty() {
        Vec::new()
    } else {
        student_mark_entries::table
            .filter(student_mark_entries::student_mark_id.eq_any(&ids))
            .load(&mut conn)?
    };
    let entry_map = map_entries(entry_rows);

    let responses = marks
        .into_iter()
        .map(|m| {
            let entries = entry_map.get(&m.id).cloned().unwrap_or_default();
            StudentMarkResponse::from_with_entries(m, entries)
        })
        .collect();
    Ok(responses)
}

pub async fn get_student_marks_by_student_id(
    pool: web::Data<AppState>,
    student_id: String,
    last_id: Option<String>,
    limit: Option<i64>,
) -> Result<Vec<StudentMarkResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut query = student_marks::table
        .filter(student_marks::student_id.eq(&student_id))
        .into_boxed();
    if let Some(last_id) = &last_id {
        query = query.filter(student_marks::id.gt(last_id));
    }
    let limit = limit.unwrap_or(10);
    let marks: Vec<StudentMark> = query
        .order(student_marks::entered_at.desc())
        .limit(limit)
        .load::<StudentMark>(&mut conn)?;

    let ids: Vec<String> = marks.iter().map(|m| m.id.clone()).collect();
    let entry_rows: Vec<StudentMarkEntry> = if ids.is_empty() {
        Vec::new()
    } else {
        student_mark_entries::table
            .filter(student_mark_entries::student_mark_id.eq_any(&ids))
            .load(&mut conn)?
    };
    let entry_map = map_entries(entry_rows);

    let responses = marks
        .into_iter()
        .map(|m| {
            let entries = entry_map.get(&m.id).cloned().unwrap_or_default();
            StudentMarkResponse::from_with_entries(m, entries)
        })
        .collect();
    Ok(responses)
}

pub async fn update_student_mark(
    pool: web::Data<AppState>,
    student_mark_id: String,
    update_request: UpdateStudentMarkRequest,
    current_user_id: String,
) -> Result<StudentMarkResponse, APIError> {
    let mut conn = pool.db_pool.get()?;
    let now = Utc::now().naive_utc();

    let existing: StudentMark = student_marks::table
        .filter(student_marks::id.eq(&student_mark_id))
        .first(&mut conn)?;

    let mut new_total = existing.total_marks;
    let mut new_percentage = existing.percentage;
    let mut entry_rows: Vec<StudentMarkEntry> = student_mark_entries::table
        .filter(student_mark_entries::student_mark_id.eq(&student_mark_id))
        .load(&mut conn)?;

    conn.transaction::<_, APIError, _>(|conn| {
        let history = StudentMarkHistory {
            id: generate_prefixed_id(conn, IdPrefix::STUDENT_MARK)?,
            student_id: existing.student_id.clone(),
            subject_id: existing.subject_id.clone(),
            assessment_type: existing.assessment_type.clone(),
            assessment_id: existing.assessment_id.clone(),
            marking_scheme_id: existing.marking_scheme_id.clone(),
            total_marks: existing.total_marks,
            percentage: existing.percentage,
            grade: existing.grade.clone(),
            grade_point: existing.grade_point,
            is_absent: existing.is_absent,
            remarks: existing.remarks.clone(),
            entered_by: existing.entered_by.clone(),
            entered_at: existing.entered_at,
            updated_by: existing.updated_by.clone(),
            updated_at: existing.updated_at,
        };
        diesel::insert_into(student_marks_history::table)
            .values(&history)
            .execute(conn)?;

        if !entry_rows.is_empty() {
            let history_entries: Vec<StudentMarkEntryHistory> = entry_rows
                .iter()
                .map(|e| Ok(StudentMarkEntryHistory {
                    id: generate_prefixed_id(conn, IdPrefix::STUDENT_MARK)?,
                    student_marks_history_id: history.id.clone(),
                    marking_scheme_part_id: e.marking_scheme_part_id.clone(),
                    marks_awarded: e.marks_awarded,
                    max_marks: e.max_marks,
                    created_at: e.created_at,
                    updated_at: e.updated_at,
                }))
                .collect::<QueryResult<Vec<_>>>()?;
            diesel::insert_into(student_mark_entries_history::table)
                .values(&history_entries)
                .execute(conn)?;
        }

        if let Some(entries) = &update_request.entries {
            let parts: Vec<MarkingSchemePart> = marking_scheme_parts::table
                .filter(marking_scheme_parts::scheme_id.eq(&existing.marking_scheme_id))
                .load(conn)?;
            let part_map: HashMap<String, MarkingSchemePart> =
                parts.into_iter().map(|p| (p.id.clone(), p)).collect();
            let (total, percent, normalized_entries) =
                compute_totals(&part_map, entries)?;
            new_total = Some(total);
            new_percentage = Some(percent);

            diesel::delete(
                student_mark_entries::table
                    .filter(student_mark_entries::student_mark_id.eq(&student_mark_id)),
            )
            .execute(conn)?;

            let mut new_entry_rows = Vec::new();
            for (e, max_marks) in normalized_entries {
                new_entry_rows.push(StudentMarkEntry {
                    id: generate_prefixed_id(conn, IdPrefix::STUDENT_MARK)?,
                    student_mark_id: student_mark_id.clone(),
                    marking_scheme_part_id: e.marking_scheme_part_id,
                    marks_awarded: e.marks_awarded,
                    max_marks,
                    created_at: now,
                    updated_at: now,
                });
            }
            entry_rows = new_entry_rows;

            if !entry_rows.is_empty() {
                diesel::insert_into(student_mark_entries::table)
                    .values(&entry_rows)
                    .execute(conn)?;
            }
        }

        let target = student_marks::table.filter(student_marks::id.eq(&student_mark_id));
        diesel::update(target)
            .set((
                student_marks::total_marks.eq(new_total),
                student_marks::percentage.eq(new_percentage),
                student_marks::is_absent.eq(update_request.is_absent.unwrap_or(existing.is_absent)),
                student_marks::remarks.eq(update_request.remarks),
                student_marks::updated_by.eq(current_user_id.clone()),
                student_marks::updated_at.eq(now),
            ))
            .execute(conn)?;
        Ok(())
    })?;

    let updated: StudentMark = student_marks::table
        .filter(student_marks::id.eq(&student_mark_id))
        .first(&mut conn)?;
    let entry_map = map_entries(entry_rows);
    let entries = entry_map
        .get(&student_mark_id)
        .cloned()
        .unwrap_or_default();
    Ok(StudentMarkResponse::from_with_entries(updated, entries))
}

pub async fn delete_student_mark(
    pool: web::Data<AppState>,
    student_mark_id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    let deleted = diesel::delete(student_marks::table.filter(student_marks::id.eq(&student_mark_id)))
        .execute(&mut conn)?;
    if deleted == 0 {
        return Err(APIError::not_found(&format!(
            "Student Mark with ID {} not found",
            student_mark_id
        )));
    }
    Ok(())
}

pub async fn bulk_create_student_marks(
    pool: web::Data<AppState>,
    bulk_request: BulkCreateStudentMarkRequest,
    current_user_id: String,
) -> Result<Vec<StudentMarkResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut created_marks = Vec::new();

    conn.transaction::<_, APIError, _>(|conn| {
        for req in bulk_request.marks {
            let parts: Vec<MarkingSchemePart> = marking_scheme_parts::table
                .filter(marking_scheme_parts::scheme_id.eq(&req.marking_scheme_id))
                .load(conn)?;
            let part_map: HashMap<String, MarkingSchemePart> =
                parts.into_iter().map(|p| (p.id.clone(), p)).collect();
            let (total_marks, percentage, normalized_entries) =
                compute_totals(&part_map, &req.entries)?;

            let now = Utc::now().naive_utc();
            let mark_id = generate_prefixed_id(conn, IdPrefix::STUDENT_MARK)?;
            let student_mark = StudentMark {
                id: mark_id.clone(),
                student_id: req.student_id,
                subject_id: req.subject_id,
                assessment_type: req.assessment_type,
                assessment_id: req.assessment_id,
                marking_scheme_id: req.marking_scheme_id,
                total_marks: Some(total_marks),
                percentage: Some(percentage),
                grade: None,
                grade_point: None,
                is_absent: req.is_absent.unwrap_or(false),
                remarks: req.remarks,
                entered_by: current_user_id.clone(),
                entered_at: now,
                updated_by: None,
                updated_at: now,
            };

            diesel::insert_into(student_marks::table)
                .values(&student_mark)
                .execute(conn)?;

            let mut entry_rows = Vec::new();
            for (e, max_marks) in normalized_entries {
                entry_rows.push(StudentMarkEntry {
                    id: generate_prefixed_id(conn, IdPrefix::STUDENT_MARK)?,
                    student_mark_id: mark_id.clone(),
                    marking_scheme_part_id: e.marking_scheme_part_id,
                    marks_awarded: e.marks_awarded,
                    max_marks,
                    created_at: now,
                    updated_at: now,
                });
            }

            if !entry_rows.is_empty() {
                diesel::insert_into(student_mark_entries::table)
                    .values(&entry_rows)
                    .execute(conn)?;
            }

            let entries = entry_rows
                .into_iter()
                .map(|e| StudentMarkEntryResponse {
                    id: e.id,
                    marking_scheme_part_id: e.marking_scheme_part_id,
                    marks_awarded: e.marks_awarded,
                    max_marks: e.max_marks,
                })
                .collect();
            created_marks.push(StudentMarkResponse::from_with_entries(student_mark, entries));
        }
        Ok(())
    })?;

    Ok(created_marks)
}

pub async fn get_student_marks_by_exam_and_class(
    pool: web::Data<AppState>,
    assessment_id: String,
    class_id: String,
) -> Result<Vec<StudentMarkResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let marks: Vec<StudentMark> = student_marks::table
        .inner_join(students::table.on(student_marks::student_id.eq(students::id)))
        .inner_join(
            student_class_assignments::table
                .on(students::id.eq(student_class_assignments::student_id)),
        )
        .filter(student_marks::assessment_id.eq(&assessment_id))
        .filter(student_class_assignments::class_id.eq(&class_id))
        .select(student_marks::all_columns)
        .order(student_marks::student_id.asc())
        .load::<StudentMark>(&mut conn)?;

    let ids: Vec<String> = marks.iter().map(|m| m.id.clone()).collect();
    let entry_rows: Vec<StudentMarkEntry> = if ids.is_empty() {
        Vec::new()
    } else {
        student_mark_entries::table
            .filter(student_mark_entries::student_mark_id.eq_any(&ids))
            .load(&mut conn)?
    };
    let entry_map = map_entries(entry_rows);

    let responses = marks
        .into_iter()
        .map(|m| {
            let entries = entry_map.get(&m.id).cloned().unwrap_or_default();
            StudentMarkResponse::from_with_entries(m, entries)
        })
        .collect();
    Ok(responses)
}
