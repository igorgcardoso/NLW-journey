use axum::async_trait;

use crate::error::AppError;

mod send_mail_task;

#[async_trait]
pub trait Task {
    async fn execute(&self) -> Result<(), AppError>;
}

pub use send_mail_task::SendMailTask;
