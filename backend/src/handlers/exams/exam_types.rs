use crate::models::exams::exam_type::{CreateExamTypeRequest, UpdateExamTypeRequest, ExamTypeResponse, ExamType, ExamTypeQuery};
use crate::services::exams::exam_types::ExamTypeService;
use crate::{create_admin_handlers, AppState};
use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

create_admin_handlers!(
    tag => "exam_types",
    entity => ExamType,
    response => ExamTypeResponse,
    query => ExamTypeQuery,
    create => CreateExamTypeRequest,
    update => UpdateExamTypeRequest,
    service => ExamTypeService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);
