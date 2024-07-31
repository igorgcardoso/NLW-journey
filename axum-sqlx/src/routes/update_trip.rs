use super::confirm_trip::Trip;
use crate::{error::AppError, AppState};
use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    #[validate(length(min = 4))]
    destination: String,
    starts_at: NaiveDateTime,
    ends_at: NaiveDateTime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    trip_id: Uuid,
}

#[debug_handler]
pub async fn update_trip(
    state: State<AppState>,
    trip_id: Path<Uuid>,
    body: Json<RequestBody>,
) -> Result<Json<ResponseBody>, AppError> {
    body.validate()?;

    // let trip_id_str = trip_id.to_string();

    let trip = query_as!(
        Trip,
        r#"
        SELECT id, destination, starts_at, ends_at, is_confirmed, created_at
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

    if body.starts_at < Local::now().naive_local() {
        return Err(AppError::BadRequest("Invalid trip start date".to_string()));
    }

    if body.ends_at < body.starts_at {
        return Err(AppError::BadRequest("Invalid trip end date".to_string()));
    }

    query!(
        r#"
        UPDATE trips
        SET destination = $1, starts_at = $2, ends_at = $3
        WHERE id = $4;
        "#,
        body.destination,
        body.starts_at,
        body.ends_at,
        *trip_id,
    )
    .execute(&*state.pool)
    .await?;

    Ok(Json(ResponseBody { trip_id: *trip_id }))
}
