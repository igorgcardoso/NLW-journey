use super::confirm_trip::Trip;
use crate::{error::AppError, AppState};
use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    #[validate(length(min = 4))]
    title: String,
    #[validate(url)]
    url: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    link_id: Uuid,
}

#[debug_handler]
pub async fn create_link(
    state: State<AppState>,
    trip_id: Path<Uuid>,
    body: Json<RequestBody>,
) -> Result<Json<ResponseBody>, AppError> {
    body.validate()?;

    let trip_id = trip_id.to_string();

    let trip = query_as!(
        Trip,
        r#"
        SELECT id, destination, starts_at, ends_at, is_confirmed, created_at
        FROM trips
        WHERE id = ?
        "#,
        trip_id,
    )
    .fetch_optional(&*state.pool)
    .await?;

    if trip.is_none() {
        return Err(AppError::BadRequest("Trip not found".to_string()));
    }

    let link_id = Uuid::new_v4();
    let link_id_str = link_id.to_string();

    query!(
        r#"
        INSERT INTO links (id, trip_id, title, url)
        VALUES (?, ?, ?, ?)
        "#,
        link_id_str,
        trip_id,
        body.title,
        body.url,
    )
    .execute(&*state.pool)
    .await?;

    Ok(Json(ResponseBody { link_id }))
}
