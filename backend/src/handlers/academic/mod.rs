use crate::create_admin_handlers;
use crate::models::academic::{
    AcademicYearQuery, AcademicYearResponse, AlStreamOptionalGroupQuery,
    AlStreamOptionalGroupResponse, AlStreamQuery, AlStreamResponse, ClassQuery, ClassResponse,
    CreateAcademicYearRequest, CreateAlStreamOptionalGroupRequest, CreateAlStreamRequest,
    CreateClassRequest, CreateGradeLevelRequest, CreateGradePeriodRequest, CreateSchoolRoomRequest,
    CreateSubjectRequest, CreateSubstitutionPlanRequest, CreateTermRequest, CreateTimetableRequest, GradeLevelQuery,
    GradeLevelResponse, GradePeriodQuery, GradePeriodResponse, SchoolRoomQuery, SchoolRoomResponse,
    SubjectQuery, SubjectResponse, SubstitutionPlan, TermQuery, TermResponse, TimetableQuery,
    TimetableResponse, UpdateAcademicYearRequest, UpdateAlStreamOptionalGroupRequest,
    UpdateAlStreamRequest, UpdateClassRequest, UpdateGradeLevelRequest, UpdateGradePeriodRequest,
    UpdateSchoolRoomRequest, UpdateSubjectRequest, UpdateSubstitutionPlanRequest,
    UpdateTermRequest, UpdateTimetableRequest,
};
use crate::services::academic::{
    AcademicYearService, AlStreamOptionalGroupService, AlStreamService, ClassService,
    GradeLevelService, GradePeriodService, SchoolRoomService, SubjectService,
    SubstitutionPlanService, TermService, TimetableService,
};
use crate::services::admin_db::AdminQuery;

create_admin_handlers!(
    tag => "academic_years",
    entity => AcademicYear,
    response => AcademicYearResponse,
    query => AcademicYearQuery,
    create => CreateAcademicYearRequest,
    update => UpdateAcademicYearRequest,
    service => AcademicYearService
);

create_admin_handlers!(
    tag => "classes",
    entity => Class,
    response => ClassResponse,
    query => ClassQuery,
    create => CreateClassRequest,
    update => UpdateClassRequest,
    service => ClassService
);

create_admin_handlers!(
    tag => "grade_levels",
    entity => GradeLevel,
    response => GradeLevelResponse,
    query => GradeLevelQuery,
    create => CreateGradeLevelRequest,
    update => UpdateGradeLevelRequest,
    service => GradeLevelService
);

create_admin_handlers!(
    tag => "subjects",
    entity => Subject,
    response => SubjectResponse,
    query => SubjectQuery,
    create => CreateSubjectRequest,
    update => UpdateSubjectRequest,
    service => SubjectService
);

create_admin_handlers!(
    tag => "terms",
    entity => Term,
    response => TermResponse,
    query => TermQuery,
    create => CreateTermRequest,
    update => UpdateTermRequest,
    service => TermService
);

create_admin_handlers!(
    tag => "timetable",
    entity => Timetable,
    response => TimetableResponse,
    query => TimetableQuery,
    create => CreateTimetableRequest,
    update => UpdateTimetableRequest,
    service => TimetableService
);

create_admin_handlers!(
    tag => "grade_periods",
    entity => GradePeriod,
    response => GradePeriodResponse,
    query => GradePeriodQuery,
    create => CreateGradePeriodRequest,
    update => UpdateGradePeriodRequest,
    service => GradePeriodService
);

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
    tag => "substitution_plans",
    entity => SubstitutionPlan,
    response => SubstitutionPlan,
    query => AdminQuery,
    create => CreateSubstitutionPlanRequest,
    update => UpdateSubstitutionPlanRequest,
    service => SubstitutionPlanService
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

