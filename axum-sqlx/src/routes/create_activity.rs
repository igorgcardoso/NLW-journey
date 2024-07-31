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
    title: String,
    occurs_at: NaiveDateTime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    activity_id: Uuid,
}

#[debug_handler]
pub async fn create_activity(
    state: State<AppState>,
    trip_id: Path<Uuid>,
    body: Json<RequestBody>,
) -> Result<Json<ResponseBody>, AppError> {
    body.validate()?;

    // let trip_id = trip_id.to_string();

    let trip = query_as!(
        Trip,
        r#"
        SELECT id, destination, starts_at, ends_at, is_confirmed, created_at
        FROM trips
        WHERE id = $1;
        "#,
        *trip_id,
    )
    .fetch_optional(&*state.pool)
    .await?;

    if trip.is_none() {
        return Err(AppError::BadRequest("Trip not found".to_string()));
    }

    let trip = trip.unwrap();

    if trip.starts_at > body.occurs_at || body.occurs_at > trip.ends_at {
        return Err(AppError::BadRequest("Invalid activity date".to_string()));
    }

    let activity_id = query!(
        r#"
        INSERT INTO activities (trip_id, title, occurs_at)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        *trip_id,
        body.title,
        body.occurs_at,
    )
    .fetch_one(&*state.pool)
    .await?;

    Ok(Json(ResponseBody {
        activity_id: activity_id.id,
    }))
}
