use super::theme::ColoredTheme;
use std::io::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Prompt that takes user input, hides it from the terminal, and returns a string
pub struct Secret {
    #[structopt(short, long)]
    /// Message for the prompt
    message: String,

    /// Enable confirmation prompt with this message
    #[structopt(short, long, requires = "error")]
    confirm: Option<String>,

    /// Error message when secrets doesn't match during confirmation
    #[structopt(short, long, requires = "confirm")]
    error: Option<String>,

    /// Allow empty secret
    #[structopt(short, long)]
    allow_empty: bool,
}

impl Secret {
    pub fn run(&self) -> Result<()> {
        let theme = ColoredTheme::default();
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
