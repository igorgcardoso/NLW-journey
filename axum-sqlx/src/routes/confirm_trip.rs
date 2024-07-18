use axum::{
    debug_handler,
    extract::{Path, State},
    response::Redirect,
};
use chrono::prelude::*;
use lettre::{
    message::{Mailbox, MultiPart, SinglePart},
    AsyncTransport,
};
use serde::Serialize;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::{error::AppError, libs::mail::get_mail_client, AppState};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    trip_id: Uuid,
}

#[allow(dead_code)]
pub(super) struct Trip {
    pub id: String,
    pub destination: String,
    pub starts_at: NaiveDateTime,
    pub ends_at: NaiveDateTime,
    pub is_confirmed: bool,
    pub created_at: NaiveDateTime,
}

#[allow(dead_code)]
pub(super) struct Participant {
    pub id: String,
    pub name: Option<String>,
    pub email: String,
    pub is_confirmed: bool,
    pub is_owner: bool,
    pub trip_id: String,
}

#[debug_handler]
pub async fn confirm_trip(
    state: State<AppState>,
    trip_id: Path<Uuid>,
) -> Result<Redirect, AppError> {
    let trip_id = trip_id.to_string();

    let redirect_url = format!("http://localhost:3000/trips/{}", trip_id);

    let trip = query_as!(
        Trip,
        r#"
        SELECT id, destination, starts_at, ends_at, is_confirmed, created_at
        FROM trips
        WHERE id = ?;
        "#,
        trip_id
    )
    .fetch_optional(&*state.pool)
    .await?;

    if trip.is_none() {
        return Err(AppError::BadRequest("Trip not found".to_string()));
    }

    let participants = query_as!(
        Participant,
        r#"
        SELECT id, name, email, is_confirmed, is_owner, trip_id
        FROM participants
        WHERE trip_id = ? AND is_owner = false;
        "#,
        trip_id
    )
    .fetch_all(&*state.pool)
    .await?;

    let trip = trip.unwrap();

    if trip.is_confirmed {
        return Ok(Redirect::to(&redirect_url));
    }

    query!(
        r#"
        UPDATE trips
        SET is_confirmed = true
        WHERE id = ?;
        "#,
        trip_id
    )
    .execute(&*state.pool)
    .await?;

    let formatted_starts_date = trip
        .starts_at
        .and_utc()
        .format_localized("%d de %B, %Y", Locale::pt_BR);
    let formatted_ends_date = trip
        .ends_at
        .and_utc()
        .format_localized("%d de %B, %Y", Locale::pt_BR);

    let mut set = tokio::task::JoinSet::new();

    participants.iter().for_each(|participant| {
        let mailer = get_mail_client().unwrap();
        let mail = lettre::Message::builder()
            .from(Mailbox::new(
                Some("Equipe Plann.er".to_string()),
                "noreply@plann.er".parse().unwrap(),
            ))
            .to(Mailbox::new(
                None,
                participant.email.parse().unwrap(),
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
                "#, &trip.destination, formatted_starts_date, formatted_ends_date, format!("http://localhost:3333/participants/{}", participant.id)).trim().to_string(),
            ))).unwrap();
        set.spawn(async move {
            mailer.send(mail).await.unwrap();
        });
    });

    while let Some(_) = set.join_next().await {}

    Ok(Redirect::to(&redirect_url))
}
