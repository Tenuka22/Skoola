use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use apistos::{api_operation, ApiComponent};
use diesel::AsChangeset;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::errors::APIError;
use crate::database::enums::{ExamScopeType, Medium};
pub use crate::models::exams::exam_structure::*;
use crate::models::ExamStructureId;
use crate::services::exams::exam_structures::{ExamStructureService, ExamStructureSubjectService};
use crate::services::exams::exam_structures;
use crate::{AppState, create_admin_handlers};

create_admin_handlers!(
    tag => "exam_structures",
    entity => ExamStructure,
    response => ExamStructure,
    query => ExamStructureQuery,
    create => CreateExamStructureRequest,
    update => UpdateExamStructureRequest,
    service => ExamStructureService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

create_admin_handlers!(
    tag => "exam_structure_subjects",
    entity => ExamStructureSubject,
    response => ExamStructureSubject,
    query => ExamStructureSubjectQuery,
    create => CreateExamStructureSubjectRequest,
    update => UpdateExamStructureSubjectRequest,
    service => ExamStructureSubjectService,
    methods => {
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateExamStructureRequest {
    pub name: String,
    pub scope_type: ExamScopeType,
    pub medium: Option<Medium>,
    pub description: Option<String>,
    pub valid_from: Option<chrono::NaiveDate>,
    pub valid_to: Option<chrono::NaiveDate>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default, AsChangeset)]
#[diesel(table_name = crate::schema::exam_structures)]
pub struct UpdateExamStructureRequest {
    pub name: Option<String>,
    pub scope_type: Option<ExamScopeType>,
    pub medium: Option<Medium>,
    pub description: Option<String>,
    pub valid_from: Option<chrono::NaiveDate>,
    pub valid_to: Option<chrono::NaiveDate>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateExamStructuresRequest {
    pub ids: Vec<String>,
    pub name: Option<String>,
    pub scope_type: Option<ExamScopeType>,
    pub medium: Option<Medium>,
    pub description: Option<String>,
    pub valid_from: Option<chrono::NaiveDate>,
    pub valid_to: Option<chrono::NaiveDate>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateExamStructureSubjectRequest {
    pub subject_id: String,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
    pub order_index: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default, AsChangeset)]
#[diesel(table_name = crate::schema::exam_structure_subjects)]
pub struct UpdateExamStructureSubjectRequest {
    pub subject_id: Option<String>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
    pub order_index: Option<i32>,
}

#[api_operation(
    summary = "Bulk delete exam structures",
    description = "Deletes multiple exam structures by IDs.",
    tag = "exam-structures",
    operation_id = "bulk_delete_exam_structures_manual"
)]
pub async fn bulk_delete_exam_structures_manual(
    data: web::Data<AppState>,
    body: web::Json<Vec<String>>,
) -> Result<HttpResponse, APIError> {
    exam_structures::bulk_delete_exam_structures(data, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_operation(
    summary = "Bulk update exam structures",
    description = "Updates multiple exam structures by IDs.",
    tag = "exam-structures",
    operation_id = "bulk_update_exam_structures_manual"
)]
pub async fn bulk_update_exam_structures_manual(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateExamStructuresRequest>,
) -> Result<HttpResponse, APIError> {
    exam_structures::bulk_update_exam_structures(data, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[api_operation(
    summary = "Create exam structure subject",
    description = "Adds a subject to an exam structure.",
    tag = "exam-structures",
    operation_id = "create_exam_structure_subject_with_logic"
)]
pub async fn create_exam_structure_subject_with_logic(
    data: web::Data<AppState>,
    path: web::Path<ExamStructureId>,
    body: web::Json<CreateExamStructureSubjectRequest>,
) -> Result<Json<ExamStructureSubject>, APIError> {
    let created = ExamStructureSubjectService::create_with_logic(
        data,
        path.into_inner().0,
        body.into_inner(),
    )
    .await?;
    Ok(Json(created))
}

