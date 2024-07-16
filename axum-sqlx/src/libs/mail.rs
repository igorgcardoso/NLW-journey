use anyhow::Result;
use lettre::{AsyncSmtpTransport, Tokio1Executor};

#[cfg(debug_assertions)]
pub fn get_mail_client() -> Result<AsyncSmtpTransport<Tokio1Executor>> {
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::from_url(
        "smtp://kody.koch:n6ddNn1CPyV6EWvEAY@smtp.ethereal.email:587?tls=required",
    )?
    .build();

    Ok(mailer)
}

#[cfg(not(debug_assertions))]
pub fn get_mail_client() -> Result<AsyncSmtpTransport<Tokio1Executor>> {
    unimplemented!()
}
