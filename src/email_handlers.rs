use anyhow::{Context, Result};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct Email<'a> {
    pub body: &'a String,
    pub from: &'a String,
    pub subject: &'a String,
    pub to: String,
}

/** Sets up connection with outlook/whatnot */
pub fn connect_smtp(creds: Credentials) -> Result<SmtpTransport> {
    let mailer = SmtpTransport::relay("smtp.outlook.com")
        .context("Failed to connect to SMTP server")?
        .credentials(creds)
        .build();

    Ok(mailer)
}

/** Sends email to a contact */
pub fn send_email(mailer: &SmtpTransport, email: &Email) -> Result<()> {
    let email = Message::builder()
        .from(email.from.parse().context("Invalid email address")?)
        .to(email.to.parse().context("Invalid email address")?)
        .subject(email.subject.to_string())
        .body(email.body.to_string())
        .unwrap();

    mailer.send(&email).context("Failed to send email")?;

    Ok(())
}
