use clap::Parser;
use dialoguer::theme::ColorfulTheme;

use std::io::Result;

/// Prompt that takes user input and returns a string.
#[derive(Debug, Parser)]
pub struct Input {
    /// Message for the prompt
    #[clap(short, long)]
    message: String,

    /// Default value for the prompt
    #[clap(short, long)]
    default: Option<String>,

    /// Allow empty input. Conflicts with `default`
    #[clap(short, long, conflicts_with = "default")]
    allow_empty: bool,
}

impl Input {
    pub fn run(&self) -> Result<()> {
        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Input::<String>::with_theme(&theme);

        input
            .with_prompt(&self.message)
            .allow_empty(self.allow_empty);

        if self.default.is_some() {
            input.default(self.default.as_ref().unwrap().to_string());
        }

        let value = input.interact_text()?;

        println!("{}", value);

        Ok(())
    }
}
