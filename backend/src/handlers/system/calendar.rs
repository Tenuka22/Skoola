use crate::models::system::calendar::{SchoolCalendarResponse, CreateSchoolCalendarRequest, UpdateSchoolCalendarRequest, SchoolCalendarQuery};
use crate::services::system::calendar::SchoolCalendarService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "school_calendar",
    entity => SchoolCalendar,
    response => SchoolCalendarResponse,
    query => SchoolCalendarQuery,
    create => CreateSchoolCalendarRequest,
    update => UpdateSchoolCalendarRequest,
    service => SchoolCalendarService,
    id_type => String,
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
