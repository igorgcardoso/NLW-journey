use super::confirm_trip::Trip;
use crate::{error::AppError, AppState};
use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use sqlx::query_as;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    links: Vec<Link>,
}

#[allow(dead_code)]
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct Link {
    pub id: String,
    pub title: String,
    pub url: String,
    pub trip_id: String,
}

#[debug_handler]
pub async fn get_links(
    state: State<AppState>,
    trip_id: Path<Uuid>,
) -> Result<Json<ResponseBody>, AppError> {
    // let trip_id = trip_id.to_string();

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

    let links = query_as!(
        Link,
        r#"
        SELECT id, title, url, trip_id
        FROM links
        WHERE trip_id = $1
        "#,
        *trip_id,
    )
    .fetch_all(&*state.pool)
    .await?;

    Ok(Json(ResponseBody { links }))
}
