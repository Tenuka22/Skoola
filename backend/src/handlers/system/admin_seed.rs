use crate::models::system::seed::{SeedResponse, CreateSeedRequest, UpdateSeedRequest, SeedQuery};
use crate::services::system::seed::SeedAdminService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "seeds",
    entity => Seed,
    response => SeedResponse,
    query => SeedQuery,
    create => CreateSeedRequest,
    update => UpdateSeedRequest,
    service => SeedAdminService,
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
