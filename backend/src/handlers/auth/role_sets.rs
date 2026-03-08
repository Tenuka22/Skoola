use crate::models::auth::role::{CreateRoleSetRequest, UpdateRoleSetRequest, RoleSetQuery};
use crate::database::tables::RoleSet;
use crate::services::auth::role_sets::RoleSetService;
use crate::{create_admin_handlers, AppState};
use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

create_admin_handlers!(
    tag => "role_sets",
    entity => RoleSet,
    response => RoleSet,
    query => RoleSetQuery,
    create => CreateRoleSetRequest,
    update => UpdateRoleSetRequest,
    service => RoleSetService,
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
