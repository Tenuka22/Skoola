use crate::models::auth::role::{CreateRoleSetRequest, UpdateRoleSetRequest, RoleSet, RoleSetQuery};
use crate::services::auth::role_sets::RoleSetService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "role_sets",
    entity => RoleSet,
    response => RoleSet,
    query => RoleSetQuery,
    create => CreateRoleSetRequest,
    update => UpdateRoleSetRequest,
    service => RoleSetService
);

