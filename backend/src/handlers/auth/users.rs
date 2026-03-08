use crate::models::auth::user::{CreateUserRequest, UpdateUserRequest, UserResponse, User, UserQuery};
use crate::services::auth::users::UserService;
use crate::{create_admin_handlers, AppState};
use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

create_admin_handlers!(
    tag => "users",
    entity => User,
    response => UserResponse,
    query => UserQuery,
    create => CreateUserRequest,
    update => UpdateUserRequest,
    service => UserService,
    methods => {
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => update_with_logic,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);
