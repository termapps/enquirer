use clap::Parser;
use dialoguer::theme::ColorfulTheme;

use std::io::Result;

/// Prompt that takes user input, hides it from the terminal, and returns a string
#[derive(Debug, Parser)]
pub struct Secret {
    /// Message for the prompt
    #[clap(short, long)]
    message: String,

    /// Enable confirmation prompt with this message
    #[clap(short, long, requires = "error")]
    confirm: Option<String>,

    /// Error message when secrets doesn't match during confirmation
    #[clap(short, long, requires = "confirm")]
    error: Option<String>,

    /// Allow empty secret
    #[clap(short, long)]
    allow_empty: bool,
}

impl Secret {
    pub fn run(&self) -> Result<()> {
        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Password::with_theme(&theme);

        input
            .with_prompt(&self.message)
            .allow_empty_password(self.allow_empty);

        if self.confirm.is_some() {
            input.with_confirmation(self.confirm.as_ref().unwrap(), self.error.as_ref().unwrap());
        }

        let value = input.interact()?;

        println!("{}", value);

        Ok(())
    }
}
