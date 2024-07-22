use super::confirm_trip::Trip;
use crate::{error::AppError, AppState};
use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use chrono::{prelude::*, Duration};
use serde::Serialize;
use sqlx::query_as;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    activities: Vec<ActivityDay>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDay {
    date: NaiveDateTime,
    activities: Vec<Activity>,
}

#[allow(dead_code)]
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct Activity {
    pub id: String,
    pub title: String,
    pub occurs_at: NaiveDateTime,
    pub trip_id: String,
}

#[debug_handler]
pub async fn get_activities(
    state: State<AppState>,
    trip_id: Path<Uuid>,
) -> Result<Json<ResponseBody>, AppError> {
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

    let trip = trip.unwrap();

    let activities = query_as!(
        Activity,
        r#"
        SELECT id, title, occurs_at, trip_id
        FROM activities
        WHERE trip_id = ?
        ORDER BY occurs_at ASC
        "#,
        trip_id,
    )
    .fetch_all(&*state.pool)
    .await?;

    let difference_in_days_between_trip_start_and_end = (trip.ends_at - trip.starts_at).num_days();
    let activities = (0..=difference_in_days_between_trip_start_and_end)
        .map(|index| {
            let date = trip.starts_at + Duration::days(index);

            ActivityDay {
                date,
                activities: activities
                    .iter()
                    .filter(|activity| activity.occurs_at.date() == date.date())
                    .cloned()
                    .collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<_>>();

    Ok(Json(ResponseBody { activities }))
}
