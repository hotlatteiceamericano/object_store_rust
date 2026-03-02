use sled::Db;

use crate::http::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub config: Config,
    pub http_client: reqwest::Client,
    pub upstash_url: String,
    pub upstash_token: String,
}

impl AppState {
    pub fn new(
        db: Db,
        config: Config,
        http_client: reqwest::Client,
        upstash_url: String,
        upstash_token: String,
    ) -> Self {
        Self {
            db,
            config,
            http_client,
            upstash_url,
            upstash_token,
        }
    }
}
