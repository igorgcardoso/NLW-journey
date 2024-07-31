use super::confirm_trip::Trip;
use crate::{error::AppError, tasks, AppState};
use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use chrono::prelude::*;
use lettre::message::{Mailbox, MultiPart, SinglePart};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    #[validate(email)]
    email: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    participant_id: Uuid,
}

#[debug_handler]
pub async fn create_invite(
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

    // let participant_id = Uuid::new_v4();
    // let participant_id_str = participant_id.to_string();

    let participant = query!(
        r#"
        INSERT INTO participants (email, trip_id)
        VALUES ($1, $2)
        RETURNING id;
        "#,
        body.email,
        *trip_id,
    )
    .fetch_one(&*state.pool)
    .await?;

    let formatted_starts_date = trip
        .starts_at
        .and_utc()
        .format_localized("%d de %B, %Y", Locale::pt_BR);
    let formatted_ends_date = trip
        .ends_at
        .and_utc()
        .format_localized("%d de %B, %Y", Locale::pt_BR);

    let mail = lettre::Message::builder()
            .from(Mailbox::new(
                Some("Equipe Plann.er".to_string()),
                "noreply@plann.er".parse().unwrap(),
            ))
            .to(Mailbox::new(
                None,
                body.email.parse().unwrap(),
            ))
            .subject(format!("Confirme sua presença na viagem para {} em {formatted_starts_date}", &trip.destination))
            .multipart(MultiPart::alternative().singlepart(SinglePart::html(
                format!(r#"
                    <div style="font-family: sans-serif; font-size: 16px; line-height: 1.6;">
                      <p>Você foi convidado(a) para participar de uma viagem para  <strong>{}</strong> nas datas de <strong>{}</strong> até <strong>{}</strong>.</p>
                      <p></p>
                      <p>Para confirmar sua presença na viagem, clique no link abaixo:</p>
                      <p></p>
                      <p>
                        <a href="{}">Confirmar viagem</a>
                      </p>
                      <p></p>
                      <p>Caso você não saiba do que se trata esse e-mail, apenas ignore esse e-mail.</p>
                    </div>
                "#, &trip.destination, formatted_starts_date, formatted_ends_date, format!("{}/participants/{}", state.config.api_base_url, participant.id)).trim().to_string(),
            ))).unwrap();
    state
        .tasks_sender
        .send(Box::new(tasks::SendMailTask::new(mail)))?;

    Ok(Json(ResponseBody {
        participant_id: participant.id,
    }))
}
