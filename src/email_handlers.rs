use anyhow::{Context, Result};
use lettre::message::{MultiPart, SinglePart, header};
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport};

pub struct Email<'a> {
    pub body: String,
    pub from: &'a str,
    pub subject: &'a str,
    pub to: String,
}

impl<'a> Email<'a> {
    pub fn new(from: &'a str, subject: &'a str) -> Self {
        Self {
            body: String::new(),
            from,
            subject,
            to: String::new(),
        }
    }
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
    attachment: Option<&SinglePart>,
) -> Result<()> {
    let to_send = match attachment {
        None => Message::builder()
            .from(email.from.parse()?)
            .to(email.to.parse()?)
            .subject(email.subject.to_string())
            .body(email.body.to_string())?,
        Some(attachment) => Message::builder()
            .from(email.from.parse()?)
            .to(email.to.parse()?)
            .subject(email.subject.to_string())
            .multipart(
                MultiPart::mixed()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(email.body.to_string()),
                    )
                    .singlepart(attachment.to_owned()),
            )?,
    };

    mailer
        .send(&to_send)
        .context(format!("Failed to send email to {}", email.to))?;

    Ok(())
}
