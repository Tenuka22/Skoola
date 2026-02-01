use std::sync::Arc;

use surrealdb::{Surreal, engine::remote::ws::Client, opt::auth::Root};

use crate::{config::Config, errors::APIError};

pub mod constants;
pub mod tables;

pub async fn init_db(config: &Config) -> Result<Arc<Surreal<Client>>, APIError> {
    let db: Surreal<Client> = Surreal::init();

    db.signin(Root {
        username: &config.db_username,
        password: &config.db_passwrod,
    })
    .await?;

    db.use_ns(&config.db_nameserver)
        .use_db(&config.db_database)
        .await?;

    return Ok(Arc::from(db));
}
