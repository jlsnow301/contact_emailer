use anyhow::{Context, Result};
use inquire::{Confirm, Password, PasswordDisplayMode, Text, validator::Validation};

/** Does this look ok? */
pub fn confirm_contacts() -> Result<()> {
    let proceed = Confirm::new("Does this sound correct?")
        .with_default(true)
        .prompt();

    match proceed {
        Ok(true) => Ok(()),
        Ok(false) => {
            println!("Exiting...");
            std::process::exit(0);
        }
        Err(_) => {
            println!("Error reading input. Exiting...");
            std::process::exit(0);
        }
    }
}

/** Prompt for email and password */
pub fn get_credentials() -> Result<(String, String)> {
    let validator = |input: &str| {
        if input.chars().count() < 3 {
            Ok(Validation::Invalid(
                "Must be at least 3 characters long".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    let email = Text::new("Enter your email:")
        .with_validator(validator)
        .prompt()
        .context("Failed to get email")?;

    let password = Password::new("Enter your password:")
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_validator(validator)
        .with_help_message("You can reveal this by keying CTRL-R.")
        .prompt()
        .context("Failed to get password")?;

    if email.is_empty() || password.is_empty() {
        return Err(anyhow::anyhow!("Email or password is empty"));
    }

    Ok((email, password))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Requires user input"]
    fn test_get_credentials() {
        let (email, password) = get_credentials().unwrap();
        assert!(!email.is_empty());
        assert!(!password.is_empty());
    }
}
