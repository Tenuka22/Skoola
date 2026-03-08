use crate::models::exams::government_exam::{
    CreateGovernmentExamRequest, UpdateGovernmentExamRequest, GovernmentExam, GovernmentExamQuery,
    CreateGovernmentExamSubjectRequest, UpdateGovernmentExamSubjectRequest, GovernmentExamSubject, GovernmentExamSubjectQuery
};
use crate::services::exams::government_exams::{GovernmentExamService, GovernmentExamSubjectService};
use crate::{create_admin_handlers, AppState};
use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

create_admin_handlers!(
    tag => "government_exams",
    entity => GovernmentExam,
    response => GovernmentExam,
    query => GovernmentExamQuery,
    create => CreateGovernmentExamRequest,
    update => UpdateGovernmentExamRequest,
    service => GovernmentExamService,
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

create_admin_handlers!(
    tag => "government_exam_subjects",
    entity => GovernmentExamSubject,
    response => GovernmentExamSubject,
    query => GovernmentExamSubjectQuery,
    create => CreateGovernmentExamSubjectRequest,
    update => UpdateGovernmentExamSubjectRequest,
    service => GovernmentExamSubjectService,
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
