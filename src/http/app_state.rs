use sled::Db;

use crate::http::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub config: Config,
}

impl AppState {
    pub fn new(db: Db, config: Config) -> Self {
        Self { db, config }
    }
}
