use crate::models::exams::exam::{CreateExamRequest, UpdateExamRequest, ExamResponse, Exam, ExamQuery};
use crate::services::exams::exams::ExamService;
use crate::{create_admin_handlers, AppState};
use crate::errors::APIError;
use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

create_admin_handlers!(
    tag => "exams",
    entity => Exam,
    response => ExamResponse,
    query => ExamQuery,
    create => CreateExamRequest,
    update => UpdateExamRequest,
    service => ExamService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => update_with_logic,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);
