use anyhow::Result;
use lettre::{AsyncSmtpTransport, Tokio1Executor};

#[cfg(debug_assertions)]
pub fn get_mail_client(connection_url: &str) -> Result<AsyncSmtpTransport<Tokio1Executor>> {
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::from_url(connection_url)?.build();

    Ok(mailer)
}
