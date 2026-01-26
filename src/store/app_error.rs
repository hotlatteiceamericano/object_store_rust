use axum::{http::StatusCode, response::IntoResponse};

// convert path: anyhow::Error -> AppError -> axum::IntoResponse
pub struct AppError {
    error: anyhow::Error,
}

// AppError -> IntoResponse
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("something went wrong: {}", self.error),
        )
            .into_response()
    }
}

// anyhow::Error -> AppError
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self {
            error: value.into(),
        }
    }
}
