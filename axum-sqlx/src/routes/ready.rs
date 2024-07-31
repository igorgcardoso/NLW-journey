use axum::{debug_handler, http::StatusCode, Json};

use crate::error::AppError;

#[debug_handler]
pub async fn ready() -> Result<(StatusCode, Json<()>), AppError> {
    Ok((StatusCode::NO_CONTENT, Json(())))
}
