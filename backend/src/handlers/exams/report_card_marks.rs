use crate::models::exams::report_card::{
    ReportCardMarkQuery, ReportCardMarkResponse,
};
use crate::services::exams::report_card_marks::ReportCardMarksService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "report_card_marks",
    entity => ReportCardMark,
    response => ReportCardMarkResponse,
    query => ReportCardMarkQuery,
    create => crate::models::exams::report_card::CreateReportCardMarkRequest,
    update => crate::models::exams::report_card::UpdateReportCardMarkRequest,
    service => ReportCardMarksService,
    methods => {
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);
