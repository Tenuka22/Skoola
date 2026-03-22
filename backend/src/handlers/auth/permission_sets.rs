use crate::models::auth::permission::{CreateUserSetRequest, UpdateUserSetRequest, UserSet, UserSetQuery};
use crate::services::auth::user_sets::UserSetService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "user_sets",
    entity => UserSet,
    response => UserSet,
    query => UserSetQuery,
    create => CreateUserSetRequest,
    update => UpdateUserSetRequest,
    service => UserSetService
);

