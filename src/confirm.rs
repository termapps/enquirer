use super::theme::ColoredTheme;
use std::io::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Prompt that returns `true` or `false` (as strings)
pub struct Confirm {
    #[structopt(short, long)]
    /// Message for the prompt
    message: String,

    #[structopt(short, long)]
    /// Default value for the prompt is `true`
    default: bool,
    // TODO: Validation
    // #[structopt(short, long)]
    // /// Command to validate the submitted value
    // validate: Option<String>,
}

impl Confirm {
    pub fn run(&self) -> Result<()> {
        let value = dialoguer::Confirm::with_theme(&ColoredTheme::default())
            .with_prompt(&self.message)
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
