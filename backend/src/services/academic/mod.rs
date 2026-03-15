pub mod class_subject_teacher;
pub mod grade_period;
pub mod substitution_plans;
pub mod timetable;

use crate::AppState;
use crate::errors::APIError;
use crate::models::academic::*;
use crate::schema::{
    academic_years, classes, grade_levels, subjects, terms as terms_schema, 
    timetable as timetable_schema, grade_periods, school_rooms, substitution_plans as substitution_plans_schema,
    al_streams, al_stream_optional_groups, al_stream_required_subjects,
    al_stream_optional_subjects, grade_subjects
};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use crate::services::admin_db::AdminQuery;
use crate::services::admin_db::BulkUpdateRequest;

impl_admin_entity_service!(
    AcademicYearService,
    academic_years::table,
    AcademicYear,
    AcademicYearResponse,
    academic_years::id,
    AcademicYearQuery,
    |q: academic_years::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(academic_years::name.like(search))
    },
    |q: academic_years::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name", "asc") => q.order(academic_years::name.asc()),
            ("name", "desc") => q.order(academic_years::name.desc()),
            _ => q.order(academic_years::created_at.desc()),
        }
    }
);

impl AcademicYearService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateAcademicYearRequest,
    ) -> Result<AcademicYearResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        if req.current.unwrap_or(false) {
            diesel::update(academic_years::table)
                .set(academic_years::current.eq(false))
                .execute(&mut conn)?;
        }
        let id = generate_prefixed_id(&mut conn, IdPrefix::ACADEMIC_YEAR)?;
        let new_item = AcademicYear {
            id,
            year_start: req.year_start,
            year_end: req.year_end,
            name: req.name,
            current: req.current.unwrap_or(false),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(pool, new_item).await
    }

    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateAcademicYearRequest,
    ) -> Result<AcademicYearResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        if req.current.unwrap_or(false) {
            diesel::update(academic_years::table)
                .set(academic_years::current.eq(false))
                .execute(&mut conn)?;
        }

        Self::generic_update(
            pool,
            id,
            (req, academic_years::updated_at.eq(Utc::now().naive_utc())),
        )
        .await
    }

    pub async fn get_current(pool: web::Data<AppState>) -> Result<AcademicYearResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let item: AcademicYear = academic_years::table
            .filter(academic_years::current.eq(true))
            .first(&mut conn)?;
        Ok(AcademicYearResponse::from(item))
    }
}

impl_admin_entity_service!(
    ClassService,
    classes::table,
    Class,
    ClassResponse,
    classes::id,
    ClassQuery,
    |q: classes::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: classes::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(classes::created_at.desc())
    }
);

impl ClassService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateClassRequest,
    ) -> Result<ClassResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::CLASS)?;
        let new_item = Class {
            id,
            grade_id: req.grade_id,
            academic_year_id: req.academic_year_id,
            class_teacher_id: req.class_teacher_id,
            medium: req.medium,
            room_id: req.room_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    GradeLevelService,
    grade_levels::table,
    GradeLevel,
    GradeLevelResponse,
    grade_levels::id,
    GradeLevelQuery,
    |q: grade_levels::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(grade_levels::grade_name.like(search))
    },
    |q: grade_levels::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("grade_name", "asc") => q.order(grade_levels::grade_name.asc()),
            ("grade_name", "desc") => q.order(grade_levels::grade_name.desc()),
            _ => q.order(grade_levels::created_at.desc()),
        }
    }
);

impl GradeLevelService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateGradeLevelRequest,
    ) -> Result<GradeLevelResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::GRADE_LEVEL)?;
        let new_item = GradeLevel {
            id,
            grade_number: req.grade_number,
            grade_name: req.grade_name,
            education_level: req.education_level,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    SubjectService,
    subjects::table,
    Subject,
    SubjectResponse,
    subjects::id,
    SubjectQuery,
    |q: subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(subjects::subject_name_en.like(search.clone())
            .or(subjects::subject_code.like(search)))
    },
    |q: subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("subject_name_en", "asc") => q.order(subjects::subject_name_en.asc()),
            ("subject_name_en", "desc") => q.order(subjects::subject_name_en.desc()),
            _ => q.order(subjects::created_at.desc()),
        }
    }
);

impl SubjectService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateSubjectRequest,
    ) -> Result<SubjectResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::SUBJECT)?;
        let new_item = Subject {
            id,
            subject_code: req.subject_code,
            subject_name_en: req.subject_name_en,
            subject_name_si: req.subject_name_si,
            subject_name_ta: req.subject_name_ta,
            is_core: req.is_core.unwrap_or(true),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    TermService,
    crate::schema::terms::table,
    Term,
    TermResponse,
    crate::schema::terms::id,
    TermQuery,
    |q: terms_schema::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: terms_schema::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(terms_schema::term_number.asc())
    }
);

impl TermService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateTermRequest,
    ) -> Result<TermResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::TERM)?;
        let new_item = Term {
            id,
            academic_year_id: req.academic_year_id,
            term_number: req.term_number,
            name: req.name,
            start_date: req.start_date,
            end_date: req.end_date,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    TimetableService,
    crate::schema::timetable::table,
    Timetable,
    TimetableResponse,
    crate::schema::timetable::id,
    TimetableQuery,
    |q: timetable_schema::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: timetable_schema::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order((timetable_schema::day_of_week.asc(), timetable_schema::start_time.asc()))
    }
);

impl TimetableService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateTimetableRequest,
    ) -> Result<TimetableResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::TIMETABLE)?;
        let new_item = Timetable {
            id,
            class_id: req.class_id,
            day_of_week: req.day_of_week,
            subject_id: req.subject_id,
            teacher_id: req.teacher_id,
            start_time: req.start_time,
            end_time: req.end_time,
            room: req.room,
            academic_year_id: req.academic_year_id,
            grade_period_id: req.grade_period_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    GradePeriodService,
    grade_periods::table,
    GradePeriod,
    GradePeriodResponse,
    grade_periods::id,
    GradePeriodQuery,
    |q: grade_periods::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: grade_periods::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(grade_periods::start_time.asc())
    }
);

impl GradePeriodService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateGradePeriodRequest,
    ) -> Result<GradePeriodResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::GRADE_PERIOD)?;
        let new_item = GradePeriod {
            id,
            grade_id: req.grade_id,
            start_time: req.start_time,
            end_time: req.end_time,
            is_break: req.is_break,
            is_optional: req.is_optional,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    SchoolRoomService,
    school_rooms::table,
    SchoolRoom,
    SchoolRoomResponse,
    school_rooms::id,
    SchoolRoomQuery,
    |q: school_rooms::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: school_rooms::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(school_rooms::created_at.desc())
    }
);

impl SchoolRoomService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateSchoolRoomRequest,
    ) -> Result<SchoolRoomResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::PROPERTY)?;
        let new_item = SchoolRoom {
            id,
            name: req.name,
            building: req.building,
            floor: req.floor,
            description: req.description,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    SubstitutionPlanService,
    crate::schema::substitution_plans::table,
    SubstitutionPlan,
    SubstitutionPlan,
    crate::schema::substitution_plans::id,
    AdminQuery,
    |q: substitution_plans_schema::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: substitution_plans_schema::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(substitution_plans_schema::created_at.desc())
    }
);

impl SubstitutionPlanService {
    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateSubstitutionPlanRequest,
    ) -> Result<SubstitutionPlan, APIError> {
        Self::generic_update(
            pool,
            id,
            (req, substitution_plans_schema::updated_at.eq(Utc::now().naive_utc())),
        )
        .await
    }

    pub async fn bulk_update_with_logic(
        pool: web::Data<AppState>,
        req: BulkUpdateRequest<UpdateSubstitutionPlanRequest>,
    ) -> Result<(), APIError> {
        use diesel::prelude::*;
        let mut conn = pool.db_pool.get()?;
        let now = Utc::now().naive_utc();

        conn.transaction::<_, APIError, _>(|conn| {
            for update in req.updates {
                diesel::update(substitution_plans_schema::table.filter(substitution_plans_schema::id.eq(update.id)))
                    .set((update.data, substitution_plans_schema::updated_at.eq(now)))
                    .execute(conn)?;
            }
            Ok(())
        })?;

        Ok(())
    }
}

impl_admin_entity_service!(
    AlStreamService,
    al_streams::table,
    AlStream,
    AlStreamResponse,
    al_streams::id,
    AlStreamQuery,
    |q: al_streams::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(al_streams::name.like(search))
    },
    |q: al_streams::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name", "asc") => q.order(al_streams::name.asc()),
            ("name", "desc") => q.order(al_streams::name.desc()),
            _ => q.order(al_streams::created_at.desc()),
        }
    }
);

impl AlStreamService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateAlStreamRequest,
    ) -> Result<AlStreamResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::AL_STREAM)?;
        let new_item = AlStream {
            id,
            name: req.name,
            description: req.description,
            version_name: req.version_name,
            start_date: req.start_date,
            end_date: req.end_date,
            is_active: req.is_active,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    AlStreamOptionalGroupService,
    al_stream_optional_groups::table,
    AlStreamOptionalGroup,
    AlStreamOptionalGroupResponse,
    al_stream_optional_groups::id,
    AlStreamOptionalGroupQuery,
    |q: al_stream_optional_groups::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(al_stream_optional_groups::group_name.like(search))
    },
    |q: al_stream_optional_groups::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("group_name", "asc") => q.order(al_stream_optional_groups::group_name.asc()),
            ("group_name", "desc") => q.order(al_stream_optional_groups::group_name.desc()),
            _ => q.order(al_stream_optional_groups::created_at.desc()),
        }
    }
);

impl AlStreamOptionalGroupService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateAlStreamOptionalGroupRequest,
    ) -> Result<AlStreamOptionalGroupResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::AL_STREAM_OPTIONAL_GROUP)?;
        let new_item = AlStreamOptionalGroup {
            id,
            stream_id: req.stream_id,
            group_name: req.group_name,
            min_select: req.min_select,
            max_select: req.max_select,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(pool, new_item).await
    }
}

pub struct AlStreamRequiredSubjectService;
impl AlStreamRequiredSubjectService {
    pub async fn create(pool: web::Data<AppState>, req: AlStreamRequiredSubject) -> Result<AlStreamRequiredSubject, APIError> {
        let mut conn = pool.db_pool.get()?;
        diesel::insert_into(al_stream_required_subjects::table)
            .values(&req)
            .execute(&mut conn)?;
        Ok(req)
    }
    pub async fn get_by_id(pool: web::Data<AppState>, stream_id: String, subject_id: String) -> Result<AlStreamRequiredSubject, APIError> {
        let mut conn = pool.db_pool.get()?;
        Ok(al_stream_required_subjects::table
            .filter(al_stream_required_subjects::stream_id.eq(stream_id))
            .filter(al_stream_required_subjects::subject_id.eq(subject_id))
            .first(&mut conn)?)
    }
    pub async fn get_all(pool: web::Data<AppState>, query: AlStreamRequiredSubjectQuery) -> Result<(Vec<AlStreamRequiredSubject>, i64, i64, Option<String>), APIError> {
        let mut conn = pool.db_pool.get()?;
        let mut q = al_stream_required_subjects::table.into_boxed();
        if let Some(stream_id) = query.stream_id {
            q = q.filter(al_stream_required_subjects::stream_id.eq(stream_id));
        }
        if let Some(subject_id) = query.subject_id {
            q = q.filter(al_stream_required_subjects::subject_id.eq(subject_id));
        }
        let items = q.load::<AlStreamRequiredSubject>(&mut conn)?;
        let total = items.len() as i64;
        Ok((items, total, 1, None))
    }
    pub async fn delete(pool: web::Data<AppState>, stream_id: String, subject_id: String) -> Result<(), APIError> {
        let mut conn = pool.db_pool.get()?;
        diesel::delete(al_stream_required_subjects::table
            .filter(al_stream_required_subjects::stream_id.eq(stream_id))
            .filter(al_stream_required_subjects::subject_id.eq(subject_id)))
            .execute(&mut conn)?;
        Ok(())
    }
}

pub struct AlStreamOptionalSubjectService;
impl AlStreamOptionalSubjectService {
    pub async fn create(pool: web::Data<AppState>, req: AlStreamOptionalSubject) -> Result<AlStreamOptionalSubject, APIError> {
        let mut conn = pool.db_pool.get()?;
        diesel::insert_into(al_stream_optional_subjects::table)
            .values(&req)
            .execute(&mut conn)?;
        Ok(req)
    }
    pub async fn get_by_id(pool: web::Data<AppState>, group_id: String, subject_id: String) -> Result<AlStreamOptionalSubject, APIError> {
        let mut conn = pool.db_pool.get()?;
        Ok(al_stream_optional_subjects::table
            .filter(al_stream_optional_subjects::group_id.eq(group_id))
            .filter(al_stream_optional_subjects::subject_id.eq(subject_id))
            .first(&mut conn)?)
    }
    pub async fn get_all(pool: web::Data<AppState>, query: AlStreamOptionalSubjectQuery) -> Result<(Vec<AlStreamOptionalSubject>, i64, i64, Option<String>), APIError> {
        let mut conn = pool.db_pool.get()?;
        let mut q = al_stream_optional_subjects::table.into_boxed();
        if let Some(group_id) = query.group_id {
            q = q.filter(al_stream_optional_subjects::group_id.eq(group_id));
        }
        if let Some(subject_id) = query.subject_id {
            q = q.filter(al_stream_optional_subjects::subject_id.eq(subject_id));
        }
        let items = q.load::<AlStreamOptionalSubject>(&mut conn)?;
        let total = items.len() as i64;
        Ok((items, total, 1, None))
    }
    pub async fn delete(pool: web::Data<AppState>, group_id: String, subject_id: String) -> Result<(), APIError> {
        let mut conn = pool.db_pool.get()?;
        diesel::delete(al_stream_optional_subjects::table
            .filter(al_stream_optional_subjects::group_id.eq(group_id))
            .filter(al_stream_optional_subjects::subject_id.eq(subject_id)))
            .execute(&mut conn)?;
        Ok(())
    }
}

pub struct GradeSubjectService;
impl GradeSubjectService {
    pub async fn create(pool: web::Data<AppState>, req: GradeSubject) -> Result<GradeSubject, APIError> {
        let mut conn = pool.db_pool.get()?;
        diesel::insert_into(grade_subjects::table)
            .values(&req)
            .execute(&mut conn)?;
        Ok(req)
    }
    pub async fn get_by_id(pool: web::Data<AppState>, grade_id: String, subject_id: String) -> Result<GradeSubject, APIError> {
        let mut conn = pool.db_pool.get()?;
        Ok(grade_subjects::table
            .filter(grade_subjects::grade_id.eq(grade_id))
            .filter(grade_subjects::subject_id.eq(subject_id))
            .first(&mut conn)?)
    }
    pub async fn get_all(pool: web::Data<AppState>, query: GradeSubjectQuery) -> Result<(Vec<GradeSubject>, i64, i64, Option<String>), APIError> {
        let mut conn = pool.db_pool.get()?;
        let mut q = grade_subjects::table.into_boxed();
        if let Some(grade_id) = query.grade_id {
            q = q.filter(grade_subjects::grade_id.eq(grade_id));
        }
        if let Some(subject_id) = query.subject_id {
            q = q.filter(grade_subjects::subject_id.eq(subject_id));
        }
        let items = q.load::<GradeSubject>(&mut conn)?;
        let total = items.len() as i64;
        Ok((items, total, 1, None))
    }
    pub async fn delete(pool: web::Data<AppState>, grade_id: String, subject_id: String) -> Result<(), APIError> {
        let mut conn = pool.db_pool.get()?;
        diesel::delete(grade_subjects::table
            .filter(grade_subjects::grade_id.eq(grade_id))
            .filter(grade_subjects::subject_id.eq(subject_id)))
            .execute(&mut conn)?;
        Ok(())
    }
}
