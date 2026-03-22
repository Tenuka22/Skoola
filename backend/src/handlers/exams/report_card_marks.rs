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
    service => ReportCardMarksService
);

