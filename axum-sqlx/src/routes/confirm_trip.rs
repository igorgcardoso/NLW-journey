use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{error::AppError, AppState};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    trip_id: Uuid,
}

#[debug_handler]
pub async fn confirm_trip(
    state: State<AppState>,
    trip_id: Path<Uuid>,
) -> Result<Json<ResponseBody>, AppError> {
    Ok(Json(ResponseBody { trip_id: *trip_id }))
}
