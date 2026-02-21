use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::services::curriculum_management;
use crate::models::curriculum_management::{CurriculumStandard, Syllabus};
use crate::models::auth::user::CurrentUser;
use crate::errors::iam::IamError;
use crate::util::permission_verification::has_permission;

use schemars::JsonSchema;
use apistos::ApiComponent;
use chrono::NaiveDateTime;

pub type Pool = web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CurriculumStandardResponse {
    pub id: String,
    pub subject_id: String,
    pub grade_level_id: String,
    pub standard_code: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<CurriculumStandard> for CurriculumStandardResponse {
    fn from(standard: CurriculumStandard) -> Self {
        CurriculumStandardResponse {
            id: standard.id,
            subject_id: standard.subject_id,
            grade_level_id: standard.grade_level_id,
            standard_code: standard.standard_code,
            description: standard.description,
            created_at: standard.created_at,
            updated_at: standard.updated_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SyllabusResponse {
    pub id: String,
    pub curriculum_standard_id: String,
    pub topic_name: String,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Syllabus> for SyllabusResponse {
    fn from(syllabus: Syllabus) -> Self {
        SyllabusResponse {
            id: syllabus.id,
            curriculum_standard_id: syllabus.curriculum_standard_id,
            topic_name: syllabus.topic_name,
            suggested_duration_hours: syllabus.suggested_duration_hours,
            description: syllabus.description,
            created_at: syllabus.created_at,
            updated_at: syllabus.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct CreateCurriculumStandardRequest {
    #[validate(length(min = 1, message = "Subject ID cannot be empty"))]
    pub subject_id: String,
    #[validate(length(min = 1, message = "Grade Level ID cannot be empty"))]
    pub grade_level_id: String,
    #[validate(length(min = 1, message = "Standard code cannot be empty"))]
    pub standard_code: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct UpdateCurriculumStandardRequest {
    #[validate(length(min = 1, message = "Subject ID cannot be empty"), custom(function = "crate::util::validation::validate_optional_string_not_empty"))]
    pub subject_id: Option<String>,
    #[validate(length(min = 1, message = "Grade Level ID cannot be empty"), custom(function = "crate::util::validation::validate_optional_string_not_empty"))]
    pub grade_level_id: Option<String>,
    #[validate(length(min = 1, message = "Standard code cannot be empty"), custom(function = "crate::util::validation::validate_optional_string_not_empty"))]
    pub standard_code: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct CreateSyllabusRequest {
    #[validate(length(min = 1, message = "Curriculum standard ID cannot be empty"))]
    pub curriculum_standard_id: String,
    #[validate(length(min = 1, message = "Topic name cannot be empty"))]
    pub topic_name: String,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct UpdateSyllabusRequest {
    #[validate(length(min = 1, message = "Topic name cannot be empty"), custom(function = "crate::util::validation::validate_optional_string_not_empty"))]
    pub topic_name: Option<String>,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
}

#[apistos::web("/curriculum-standards", post, 
    operation_id = "create_curriculum_standard", 
    tag = "Curriculum Management", 
    request_body(content = "CreateCurriculumStandardRequest", description = "Create curriculum standard request"), 
    responses( (status = 201, description = "Curriculum standard created", content = "CurriculumStandardResponse") ) 
)]
pub async fn create_curriculum_standard(pool: Pool, current_user: CurrentUser, req: web::Json<CreateCurriculumStandardRequest>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "curriculum:create")?;

    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let standard = web::block(move || {
        curriculum_management::create_curriculum_standard(&mut conn, req.subject_id.clone(), req.grade_level_id.clone(), req.standard_code.clone(), req.description.clone())
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Created().json(CurriculumStandardResponse::from(standard)))
}

#[apistos::web("/curriculum-standards/{standard_id}", get, 
    operation_id = "get_curriculum_standard_by_id", 
    tag = "Curriculum Management", 
    responses( (status = 200, description = "Curriculum standard retrieved", content = "CurriculumStandardResponse"), (status = 404, description = "Curriculum standard not found") ) 
)]
pub async fn get_curriculum_standard_by_id(pool: Pool, current_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "curriculum:view")?;

    let standard_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let standard = web::block(move || {
        curriculum_management::get_curriculum_standard_by_id(&mut conn, &standard_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    match standard {
        Some(s) => Ok(HttpResponse::Ok().json(CurriculumStandardResponse::from(s))),
        None => Err(IamError::NotFound("Curriculum standard not found".to_string())),
    }
}

#[apistos::web("/curriculum-standards", get, 
    operation_id = "get_all_curriculum_standards", 
    tag = "Curriculum Management", 
    responses( (status = 200, description = "Curriculum standards retrieved", content = "Vec<CurriculumStandardResponse>") ) 
)]
pub async fn get_all_curriculum_standards(pool: Pool, current_user: CurrentUser) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "curriculum:view")?;

    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let standards = web::block(move || {
        curriculum_management::get_all_curriculum_standards(&mut conn)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Ok().json(standards.into_iter().map(CurriculumStandardResponse::from).collect::<Vec<_>>()))
}

#[apistos::web("/curriculum-standards/{standard_id}", put, 
    operation_id = "update_curriculum_standard", 
    tag = "Curriculum Management", 
    request_body(content = "UpdateCurriculumStandardRequest", description = "Update curriculum standard request"), 
    responses( (status = 200, description = "Curriculum standard updated", content = "CurriculumStandardResponse"), (status = 404, description = "Curriculum standard not found") ) 
)]
pub async fn update_curriculum_standard(pool: Pool, current_user: CurrentUser, path: web::Path<String>, req: web::Json<UpdateCurriculumStandardRequest>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "curriculum:update")?;

    let standard_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let updated_standard = web::block(move || {
        curriculum_management::update_curriculum_standard(&mut conn, &standard_id, req.subject_id.clone(), req.grade_level_id.clone(), req.standard_code.clone(), req.description.clone())
    })
    .await?
    .map_err(IamError::ServiceError)?;

    match updated_standard {
        Some(s) => Ok(HttpResponse::Ok().json(CurriculumStandardResponse::from(s))),
        None => Err(IamError::NotFound("Curriculum standard not found".to_string())),
    }
}

#[apistos::web("/curriculum-standards/{standard_id}", delete, 
    operation_id = "delete_curriculum_standard", 
    tag = "Curriculum Management", 
    responses( (status = 204, description = "Curriculum standard deleted"), (status = 404, description = "Curriculum standard not found") ) 
)]
pub async fn delete_curriculum_standard(pool: Pool, current_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "curriculum:delete")?;

    let standard_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let num_deleted = web::block(move || {
        curriculum_management::delete_curriculum_standard(&mut conn, &standard_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    if num_deleted > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(IamError::NotFound("Curriculum standard not found".to_string()))
    }
}

#[apistos::web("/syllabus", post, 
    operation_id = "create_syllabus_topic", 
    tag = "Curriculum Management", 
    request_body(content = "CreateSyllabusRequest", description = "Create syllabus topic request"), 
    responses( (status = 201, description = "Syllabus topic created", content = "SyllabusResponse") ) 
)]
pub async fn create_syllabus_topic(pool: Pool, current_user: CurrentUser, req: web::Json<CreateSyllabusRequest>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "syllabus:create")?;

    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let syllabus_topic = web::block(move || {
        curriculum_management::create_syllabus_topic(&mut conn, req.curriculum_standard_id.clone(), req.topic_name.clone(), req.suggested_duration_hours, req.description.clone())
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Created().json(SyllabusResponse::from(syllabus_topic)))
}

#[apistos::web("/syllabus/{syllabus_id}", get, 
    operation_id = "get_syllabus_topic_by_id", 
    tag = "Curriculum Management", 
    responses( (status = 200, description = "Syllabus topic retrieved", content = "SyllabusResponse"), (status = 404, description = "Syllabus topic not found") ) 
)]
pub async fn get_syllabus_topic_by_id(pool: Pool, current_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "syllabus:view")?;

    let syllabus_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let syllabus_topic = web::block(move || {
        curriculum_management::get_syllabus_topic_by_id(&mut conn, &syllabus_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    match syllabus_topic {
        Some(s) => Ok(HttpResponse::Ok().json(SyllabusResponse::from(s))),
        None => Err(IamError::NotFound("Syllabus topic not found".to_string())),
    }
}

#[apistos::web("/curriculum-standards/{standard_id}/syllabus", get, 
    operation_id = "get_syllabus_topics_for_standard", 
    tag = "Curriculum Management", 
    responses( (status = 200, description = "Syllabus topics retrieved", content = "Vec<SyllabusResponse>") ) 
)]
pub async fn get_syllabus_topics_for_standard(pool: Pool, current_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "syllabus:view")?;

    let standard_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let syllabus_topics = web::block(move || {
        curriculum_management::get_syllabus_topics_for_standard(&mut conn, &standard_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Ok().json(syllabus_topics.into_iter().map(SyllabusResponse::from).collect::<Vec<_>>()))
}

#[apistos::web("/syllabus/{syllabus_id}", put, 
    operation_id = "update_syllabus_topic", 
    tag = "Curriculum Management", 
    request_body(content = "UpdateSyllabusRequest", description = "Update syllabus topic request"), 
    responses( (status = 200, description = "Syllabus topic updated", content = "SyllabusResponse"), (status = 404, description = "Syllabus topic not found") ) 
)]
pub async fn update_syllabus_topic(pool: Pool, current_user: CurrentUser, path: web::Path<String>, req: web::Json<UpdateSyllabusRequest>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "syllabus:update")?;

    let syllabus_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let updated_syllabus_topic = web::block(move || {
        curriculum_management::update_syllabus_topic(&mut conn, &syllabus_id, req.topic_name.clone(), req.suggested_duration_hours, req.description.clone())
    })
    .await?
    .map_err(IamError::ServiceError)?;

    match updated_syllabus_topic {
        Some(s) => Ok(HttpResponse::Ok().json(SyllabusResponse::from(s))),
        None => Err(IamError::NotFound("Syllabus topic not found".to_string())),
    }
}

#[apistos::web("/syllabus/{syllabus_id}", delete, 
    operation_id = "delete_syllabus_topic", 
    tag = "Curriculum Management", 
    responses( (status = 204, description = "Syllabus topic deleted"), (status = 404, description = "Syllabus topic not found") ) 
)]
pub async fn delete_syllabus_topic(pool: Pool, current_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "syllabus:delete")?;

    let syllabus_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let num_deleted = web::block(move || {
        curriculum_management::delete_syllabus_topic(&mut conn, &syllabus_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    if num_deleted > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(IamError::NotFound("Syllabus topic not found".to_string()))
    }
}
