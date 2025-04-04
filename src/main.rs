use anyhow::Result;
use email_handlers::{Email, connect_smtp, send_email};
use file_handlers::{get_attachment, get_contacts, load_email_template};
use lettre::transport::smtp::authentication::Credentials;
use owo_colors::OwoColorize;
use user_input::{confirm_contacts, get_credentials};

mod email_handlers;
mod file_handlers;
mod user_input;

fn main() -> Result<()> {
    println!("{}", "Welcome :).".blue());
    println!("Finding contacts...");

    let contacts = get_contacts()?;
    println!("contacts.csv: {} found.", contacts.len());

    confirm_contacts()?;

    let attachment = get_attachment()?;
    if attachment.is_some() {
        println!("Attachment found.");
    }

    let (subject, body) = load_email_template()?;
    println!("Email template loaded.");

    let (from, password) = get_credentials()?;
    let credentials = Credentials::new(from.clone(), password);
    println!("Loaded credentials.");

    println!("\nConnecting to email server...");
    let mailer = connect_smtp(credentials)?;
    println!("{}", "Connected.".green());

    let length = contacts.len();
    println!("Sending...");

    let mut email = Email::new(&from, &subject);

    for (i, contact) in contacts.iter().enumerate() {
        email.to = contact.email.clone();
        email.body = body.replace("{{name}}", &contact.name);

        send_email(&mailer, &email, attachment.as_ref())?;

        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("{} / {}", i + 1, length);
    }

    println!("\n{} emails sent.", length.green());
    println!("\nPress any key to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());

    Ok(())
}
