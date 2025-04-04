use anyhow::{Context, Result};
use csv::Reader;
use eml_parser::{EmlParser, eml::Eml};
use lettre::message::header::ContentType;
use lettre::message::{Attachment, SinglePart};
use std::{fs, fs::File, path::Path};

#[derive(Debug)]
pub struct Contact {
    pub name: String,
    pub email: String,
}

const FIVE_MB: u64 = 5 * 1024 * 1024; // 5MB 
const TEN_MB: u64 = 10 * 1024 * 1024; // 10MB 

/** Gets contacts from the contacts.csv file */
pub fn get_contacts() -> Result<Vec<Contact>> {
    let path = Path::new("contacts.csv");
    let file = File::open(path).context("Failed to open contacts.csv")?;

    let mut reader = Reader::from_reader(file);
    let mut contacts = Vec::new();

    for result in reader.records() {
        let record = result.context("Failed to read record")?;
        if record.len() >= 2 {
            contacts.push(Contact {
                name: record[0].to_string(),
                email: record[1].to_string(),
            })
        }
    }

    Ok(contacts)
}

/** Gets the parsed template from a template.eml file */
fn get_parsed_template() -> Result<Eml> {
    let mut template =
        EmlParser::from_file("template.eml").context("Failed to load template.eml")?;

    let parsed = template.parse().context("Failed to parse template email")?;

    Ok(parsed)
}

/** Gets the subject and body from an email template */
fn get_email_info(parsed: Eml) -> Result<(String, String)> {
    let subject = parsed
        .subject
        .ok_or_else(|| anyhow::anyhow!("Failed to get subject"))?;
    let body = parsed
        .body
        .ok_or_else(|| anyhow::anyhow!("Failed to get body"))?;

    if subject.is_empty() || body.is_empty() {
        return Err(anyhow::anyhow!("Subject or body is empty"));
    }

    Ok((subject, body))
}

/** Loads an email template from a template.eml file */
pub fn load_email_template() -> Result<(String, String)> {
    check_file_size("template.eml", TEN_MB)?;

    let parsed = get_parsed_template()?;
    let (subject, body) = get_email_info(parsed)?;

    Ok((subject, body))
}

/** Gets a single attachment from the attachments directory */
pub fn get_attachment() -> Result<Option<SinglePart>> {
    let attachments: Vec<_> = fs::read_dir("attachments")
        .context("Failed to read attachments directory")?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    if attachments.len() > 1 {
        return Err(anyhow::anyhow!("More than one attachment found"));
    }

    match attachments.first() {
        None => Ok(None),
        Some(found) => {
            check_file_size(found.path().to_str().unwrap(), FIVE_MB)?;

            let attachment = Attachment::new(found.file_name().to_string_lossy().into_owned())
                .body(
                    fs::read(found.path()).context("Failed to read attachment")?,
                    ContentType::parse("application/pdf")?,
                );

            Ok(Some(attachment))
        }
    }
}

/** Checks if files are too large */
pub fn check_file_size(path: &str, max_size: u64) -> Result<()> {
    let metadata = fs::metadata(path).context("Failed to get metadata")?;
    let size = metadata.len();

    if size > max_size {
        return Err(anyhow::anyhow!(
            "A file is too large. Max size is {}.",
            if max_size == FIVE_MB { "5MB" } else { "10MB" }
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_email_info() {
        let mut template = EmlParser::from_file("src/test/test.eml").unwrap();
        let parsed = template.parse().unwrap();

        let (subject, body) = get_email_info(parsed).unwrap();
        assert!(!subject.is_empty());
        assert!(!body.is_empty());
        assert!(body.contains("test email"));
    }

    #[test]
    fn test_get_contacts() {
        let contacts = get_contacts().unwrap();
        assert!(!contacts.is_empty());
    }
}
