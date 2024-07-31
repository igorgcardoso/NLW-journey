use axum::{debug_handler, extract::State, Json};
use chrono::prelude::*;
use lettre::message::{Mailbox, MultiPart, SinglePart};
use serde::{Deserialize, Serialize};
use sqlx::query;
use uuid::Uuid;
use validator::{Validate, ValidateEmail, ValidationError};

use crate::{error::AppError, tasks, AppState};

#[derive(Deserialize, Validate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    #[validate(length(min = 4))]
    destination: String,
    starts_at: NaiveDateTime,
    ends_at: NaiveDateTime,
    owner_name: String,
    #[validate(email)]
    owner_email: String,
    #[validate(custom(function = "validate_emails"))]
    emails_to_invite: Vec<String>,
}

fn validate_emails(emails: &Vec<String>) -> Result<(), ValidationError> {
    for email in emails.iter() {
        if !email.validate_email() {
            return Err(ValidationError::new("Invalid email"));
        }
    }

    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    trip_id: Uuid,
}

#[debug_handler]
pub async fn create_trip(
    state: State<AppState>,
    body: Json<RequestBody>,
) -> Result<Json<ResponseBody>, AppError> {
    body.validate()?;

    if body.starts_at < Local::now().naive_local() {
        return Err(AppError::BadRequest("Invalid trip start date".to_string()));
    }

    if body.ends_at < body.starts_at {
        return Err(AppError::BadRequest("Invalid trip end date".to_string()));
    }

    // let trip_id = Uuid::new_v4();
    // let id_str = trip_id.to_string();

    // let participant_id = Uuid::new_v4();
    // let participant_id_str = participant_id.to_string();

    let mut tx = state.pool.begin().await?;

    let trip = query!(
        r#"
        INSERT INTO trips (destination, starts_at, ends_at)
        VALUES ($1, $2, $3)
        RETURNING id;
        "#,
        body.destination,
        body.starts_at,
        body.ends_at,
    )
    .fetch_one(&mut *tx)
    .await?;

    match query!(
        r#"
        INSERT INTO participants (name, email, is_confirmed, is_owner, trip_id)
        VALUES ($1, $2, true, true, $3);
        "#,
        body.owner_name,
        body.owner_email,
        trip.id
    )
    .execute(&mut *tx)
    .await
    {
        Ok(_) => (),
        Err(err) => {
            tx.rollback().await?;
            return Err(AppError::InternalServerError(err.to_string()));
        }
    }

    for email in body.emails_to_invite.iter() {
        match query!(
            r#"
            INSERT INTO participants (email, trip_id)
            VALUES ($1, $2);
            "#,
            email,
            trip.id
        )
        .execute(&mut *tx)
        .await
        {
            Ok(_) => (),
            Err(err) => {
                tx.rollback().await?;
                return Err(AppError::InternalServerError(err.to_string()));
            }
        }
    }

    tx.commit().await?;

    let formatted_starts_date = body
        .starts_at
        .and_utc()
        .format_localized("%d de %B, %Y", Locale::pt_BR);
    let formatted_ends_date = body
        .ends_at
        .and_utc()
        .format_localized("%d de %B, %Y", Locale::pt_BR);

    let mail = lettre::Message::builder()
        .from(Mailbox::new(
            Some("Equipe Plann.er".to_string()),
            "noreply@plann.er".parse().unwrap(),
        ))
        .to(Mailbox::new(
            Some(body.owner_name.clone()),
            body.owner_email.parse().unwrap(),
        ))
        .subject(format!("Confirme sua viagem para {} em {formatted_starts_date}", body.destination))
        .multipart(MultiPart::alternative().singlepart(SinglePart::html(
            format!(r#"
                <div style="font-family: sans-serif; font-size: 16px; line-height: 1.6;">
                  <p>Você solicitou a criação de uma viagem para <strong>{}</strong> nas datas de <strong>{}</strong> até <strong>{}</strong>.</p>
                  <p></p>
                  <p>Para confirmar sua viagem, clique no link abaixo:</p>
                  <p></p>
                  <p>
                    <a href="{}">Confirmar viagem</a>
                  </p>
                  <p></p>
                  <p>Caso você não saiba do que se trata esse e-mail, apenas ignore esse e-mail.</p>
                </div>
            "#, body.destination.clone(), formatted_starts_date, formatted_ends_date, format!("{}/trips/{}/confirm", trip.id,  state.config.api_base_url)).trim().to_string(),
        )))?;

    state
        .tasks_sender
        .send(Box::new(tasks::SendMailTask::new(mail)))?;

    Ok(Json(ResponseBody { trip_id: trip.id }))
}
