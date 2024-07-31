use super::get_participants::Participant;
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
    participant: Participant,
}

#[debug_handler]
pub async fn get_participant(
    state: State<AppState>,
    participant_id: Path<Uuid>,
) -> Result<Json<ResponseBody>, AppError> {
    // let participant_id = participant_id.to_string();

    let participant = query_as!(
        Participant,
        r#"
        SELECT id, name, email, is_confirmed
        FROM participants
        WHERE id = $1
        "#,
        *participant_id,
    )
    .fetch_optional(&*state.pool)
    .await?;

    if participant.is_none() {
        return Err(AppError::BadRequest("Participant not found".to_string()));
    }

    let participant = participant.unwrap();

    Ok(Json(ResponseBody { participant }))
}
