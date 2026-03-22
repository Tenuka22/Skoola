use crate::models::auth::user::{UserResponse, UserQuery, CreateUserRequest};
use crate::services::auth::users::UserService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "users",
    entity => User,
    response => UserResponse,
    query => UserQuery,
    create => CreateUserRequest,
    update => User,
    service => UserService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

