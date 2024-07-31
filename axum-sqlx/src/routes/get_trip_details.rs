use crate::{error::AppError, AppState};
use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::query_as;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    trip: Trip,
}

#[allow(dead_code)]
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct Trip {
    pub id: String,
    pub destination: String,
    pub starts_at: NaiveDateTime,
    pub ends_at: NaiveDateTime,
    pub is_confirmed: bool,
}

#[debug_handler]
pub async fn get_trip_details(
    state: State<AppState>,
    trip_id: Path<Uuid>,
) -> Result<Json<ResponseBody>, AppError> {
    // let trip_id = trip_id.to_string();

    let trip = query_as!(
        Trip,
        r#"
        SELECT id, destination, starts_at, ends_at, is_confirmed
        FROM trips
        WHERE id = $1
        "#,
        *trip_id,
    )
    .fetch_optional(&*state.pool)
    .await?;

    if trip.is_none() {
        return Err(AppError::BadRequest("Trip not found".to_string()));
    }

    let trip = trip.unwrap();

    Ok(Json(ResponseBody { trip }))
}
