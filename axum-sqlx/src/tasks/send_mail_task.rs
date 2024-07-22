use axum::async_trait;
use lettre::{AsyncTransport, Message};

use crate::{error::AppError, libs::mail::get_mail_client};

use super::Task;

#[derive(Clone)]
pub struct SendMailTask {
    pub mail: Message,
}

impl SendMailTask {
    pub fn new(mail: Message) -> Self {
        Self { mail }
    }
}

#[async_trait]
impl Task for SendMailTask {
    async fn execute(&self) -> Result<(), AppError> {
        let email_connection_url = std::env::var("EMAIL_CONNECTION_URL").unwrap();
        let mailer = get_mail_client(&email_connection_url).unwrap();

        mailer.send(self.mail.clone()).await?;

        Ok(())
    }
}
