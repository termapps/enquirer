use dialoguer::theme::ColorfulTheme;
use structopt::StructOpt;

use std::io::Result;

/// Prompt that returns `true` or `false` (as strings)
#[derive(Debug, StructOpt)]
pub struct Confirm {
    /// Message for the prompt
    #[structopt(short, long)]
    message: String,

    /// Makes the prompt cancellable with 'Esc' or 'q'
    #[structopt(short, long)]
    cancel: bool,

    /// Sets the default value for the prompt as `true`
    #[structopt(short, long)]
    default: bool,
    // TODO: Validation
    // #[structopt(short, long)]
    // /// Command to validate the submitted value
    // validate: Option<String>,
}

impl Confirm {
    pub fn run(&self) -> Result<()> {
        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Confirm::with_theme(&theme);

        input.with_prompt(&self.message).default(self.default);

        let ret = if self.cancel {
            input.interact_opt()?
        } else {
            Some(input.interact()?)
        };

        let value = match ret {
            Some(value) => value,
            None => std::process::exit(1),
        };

        if value {
            println!("true");
        } else {
            println!("false");
        }

        Ok(())
    }
}
