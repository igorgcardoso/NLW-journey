use super::confirm_trip::Participant;
use crate::{error::AppError, AppState};
use axum::{
    debug_handler,
    extract::{Path, State},
    response::Redirect,
};
use chrono::prelude::*;
use serde::Serialize;
use sqlx::{query, query_as};
use uuid::Uuid;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    trip_id: Uuid,
}

#[allow(dead_code)]
struct Trip {
    id: Uuid,
    destination: String,
    starts_at: NaiveDateTime,
    ends_at: NaiveDateTime,
    is_confirmed: bool,
    created_at: NaiveDateTime,
}

#[debug_handler]
pub async fn confirm_participant(
    state: State<AppState>,
    participant_id: Path<Uuid>,
) -> Result<Redirect, AppError> {
    // let participant_id = participant_id.to_string();

    let participant = query_as!(
        Participant,
        r#"
        SELECT id, name, email, is_confirmed, is_owner, trip_id
        FROM participants
        WHERE id = $1;
        "#,
        *participant_id
    )
    .fetch_optional(&*state.pool)
    .await?;

    if participant.is_none() {
        return Err(AppError::BadRequest("Participant not found".to_string()));
    }

    let participant = participant.unwrap();

    let redirect_url = format!(
        "{}/trips/{}",
        state.config.web_base_url, participant.trip_id
    );

    if participant.is_confirmed {
        return Ok(Redirect::to(&redirect_url));
    }

    query!(
        r#"
        UPDATE participants
        SET is_confirmed = true
        WHERE id = $1;
        "#,
        *participant_id
    )
    .execute(&*state.pool)
    .await?;

    Ok(Redirect::to(&redirect_url))
}
