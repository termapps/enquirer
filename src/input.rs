use dialoguer::theme::ColorfulTheme;
use structopt::StructOpt;

use std::io::Result;

/// Prompt that takes user input and returns a string.
#[derive(Debug, StructOpt)]
pub struct Input {
    /// Message for the prompt
    #[structopt(short, long)]
    message: String,

    /// Default value for the prompt
    #[structopt(short, long)]
    default: Option<String>,

    /// Allow empty input. Conflicts with `default`
    #[structopt(short, long, conflicts_with = "default")]
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
