use super::theme::ColoredTheme;
use dialoguer::Confirmation;
use std::io::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Prompt that returns `true` or `false`
pub struct Confirm {
    #[structopt(short, long)]
    /// Message for the prompt
    message: String,

    #[structopt(short, long)]
    /// Default value for the prompt is `true`
    default: bool,

    #[structopt(short, long)]
    /// Command to validate the submitted value
    validate: Option<String>,
}

impl Confirm {
    pub fn run(&self) -> Result<()> {
        let value = Confirmation::with_theme(&ColoredTheme::default())
            .with_text(&self.message)
            .default(self.default)
            .interact()?;

        if value {
            println!("true");
        } else {
            println!("false");
        }

        Ok(())
    }
}
