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
    service => UserSetService,
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
