use super::theme::ColoredTheme;
use dialoguer;
use std::io::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Prompt that takes user input and returns a string.
pub struct Input {
    #[structopt(short, long)]
    /// Message for the prompt
    message: String,

    #[structopt(short, long)]
    /// Default value for the prompt
    default: Option<String>,

    /// Allow empty input. Conflicts with `default`
    #[structopt(short, long, conflicts_with = "default")]
    allow_empty: bool,
}

impl Input {
    pub fn run(&self) -> Result<()> {
        let theme = ColoredTheme::default();
        let mut input = dialoguer::Input::<String>::with_theme(&theme);

        input
            .with_prompt(&self.message)
            .allow_empty(self.allow_empty);

        if self.default.is_some() {
            input.default(self.default.as_ref().unwrap().to_string());
        }

        let value = input.interact()?;

        println!("{}", value);

        Ok(())
    }
}
