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
    service => SeedAdminService
);

