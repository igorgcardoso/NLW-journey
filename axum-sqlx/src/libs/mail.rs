use anyhow::Result;
use lettre::{AsyncSmtpTransport, Tokio1Executor};

#[cfg(debug_assertions)]
pub fn get_mail_client() -> Result<AsyncSmtpTransport<Tokio1Executor>> {
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::from_url(
        "smtp://alba.lemke:8jwXaxeMBrEnWjr25k@smtp.ethereal.email:587?tls=required",
    )?
    .build();

    Ok(mailer)
}
