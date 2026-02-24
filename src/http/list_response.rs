use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListResponse {
    pub bucket: String,
    pub prefix: Option<String>,
    pub filename: Option<String>,
    pub object_names: Vec<String>,
}

impl ListResponse {
    pub fn new(
        bucket: String,
        prefix: Option<String>,
        filename: Option<String>,
        object_names: Vec<String>,
    ) -> Self {
        Self {
            bucket,
            prefix,
            filename,
            object_names,
        }
    }
}

impl IntoResponse for ListResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self).into_response()).into_response()
    }
}
