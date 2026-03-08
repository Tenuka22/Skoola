use crate::models::auth::user::{UpdateUserRequest, UserResponse, UserQuery};
use crate::services::auth::users::UserService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "users",
    entity => User,
    response => UserResponse,
    query => UserQuery,
    create => User, // Placeholder
    update => UpdateUserRequest,
    service => UserService,
    methods => {
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);
