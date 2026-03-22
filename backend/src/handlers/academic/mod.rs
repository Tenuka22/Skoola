use crate::create_admin_handlers;
use crate::models::academic::{
    AlStreamOptionalGroupQuery,
    AlStreamOptionalGroupResponse, AlStreamQuery, AlStreamResponse,
    CreateAlStreamOptionalGroupRequest, CreateAlStreamRequest,
    CreateSchoolRoomRequest,
    SchoolRoomQuery, SchoolRoomResponse,
    UpdateAlStreamOptionalGroupRequest,
    UpdateAlStreamRequest,
    UpdateSchoolRoomRequest,
};
use crate::services::academic::{
    AlStreamOptionalGroupService, AlStreamService,
    SchoolRoomService,
};

pub mod academic_year;
pub mod class;
pub mod class_subject_teacher;
pub mod grade_level;
pub mod grade_period;
pub mod subject;
pub mod substitution_plans;
pub mod teacher_assignments;
pub mod terms;
pub mod timetable;

pub use academic_year::*;
pub use class::*;
pub use grade_level::*;
pub use grade_period::*;
pub use subject::*;
pub use substitution_plans::*;
pub use terms::*;
pub use timetable::*;

create_admin_handlers!(
    tag => "school_rooms",
    entity => SchoolRoom,
    response => SchoolRoomResponse,
    query => SchoolRoomQuery,
    create => CreateSchoolRoomRequest,
    update => UpdateSchoolRoomRequest,
    service => SchoolRoomService
);

create_admin_handlers!(
    tag => "al_streams",
    entity => AlStream,
    response => AlStreamResponse,
    query => AlStreamQuery,
    create => CreateAlStreamRequest,
    update => UpdateAlStreamRequest,
    service => AlStreamService
);

create_admin_handlers!(
    tag => "al_stream_optional_groups",
    entity => AlStreamOptionalGroup,
    response => AlStreamOptionalGroupResponse,
    query => AlStreamOptionalGroupQuery,
    create => CreateAlStreamOptionalGroupRequest,
    update => UpdateAlStreamOptionalGroupRequest,
    service => AlStreamOptionalGroupService
);


