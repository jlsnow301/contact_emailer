use anyhow::{Context, Result};
use lettre::message::{MultiPart, SinglePart, header};
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport};

pub struct Email<'a> {
    pub body: &'a String,
    pub from: &'a String,
    pub subject: &'a String,
    pub to: String,
}

/** Sets up connection with outlook/whatnot */
pub fn connect_smtp(creds: Credentials) -> Result<SmtpTransport> {
    let server = "smtp.office365.com";

    let mailer = SmtpTransport::relay(server)
        .context("Failed to connect to SMTP server")?
        .port(587)
        .credentials(creds)
        .authentication(vec![Mechanism::Login])
        .tls(Tls::Required(TlsParameters::new(server.to_string())?))
        .build();

    Ok(mailer)
}

/** Sends email to a contact */
pub fn send_email(
    mailer: &SmtpTransport,
    email: &Email,
    attachment: &Option<SinglePart>,
) -> Result<()> {
    let to_send = if attachment.is_none() {
        Message::builder()
            .from(email.from.parse().context("Invalid email address")?)
            .to(email.to.parse().context("Invalid email address")?)
            .subject(email.subject.to_string())
            .body(email.body.to_string())?
    } else {
        let attachment = attachment.as_ref().unwrap();

        Message::builder()
            .from(email.from.parse().context("Invalid email address")?)
            .to(email.to.parse().context("Invalid email address")?)
            .subject(email.subject.to_string())
            .multipart(
                MultiPart::mixed()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(email.body.to_string()),
                    )
                    .singlepart(attachment.clone()),
            )?
    };

    mailer.send(&to_send).context("Failed to send email")?;

    Ok(())
}
