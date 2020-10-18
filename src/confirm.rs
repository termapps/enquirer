use dialoguer::theme::ColorfulTheme;
use structopt::StructOpt;

use std::io::Result;

/// Prompt that returns `true` or `false` (as strings)
#[derive(Debug, StructOpt)]
pub struct Confirm {
    /// Message for the prompt
    #[structopt(short, long)]
    message: String,

    /// Default value for the prompt is `true`
    #[structopt(short, long)]
    default: bool,
    // TODO: Validation
    // #[structopt(short, long)]
    // /// Command to validate the submitted value
    // validate: Option<String>,
}

impl Confirm {
    pub fn run(&self) -> Result<()> {
        let value = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
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
