use anyhow::{Context, Result};
use dotenv::dotenv;
use inquire::Confirm;
use std::env;

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
    dotenv().ok();

    let email = env::var("EMAIL").context("Our email is not set")?;
    let password = env::var("PASS").context("Our password is not set")?;

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
