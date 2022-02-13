use clap::Parser;
use dialoguer::theme::ColorfulTheme;

use std::io::Result;

/// Prompt that allows the user to sort items in a list
#[derive(Debug, Parser)]
pub struct Sort {
    /// Message for the prompt
    #[clap(short, long)]
    message: String,

    /// Makes the prompt cancellable with 'Esc' or 'q'
    #[clap(short, long)]
    cancel: bool,

    /// Makes the prompt return default order as given if --cancel option is present
    #[clap(short = 'd', long = "default", requires = "cancel")]
    return_default: bool,

    /// Returns index of the sorted items instead of items itself
    #[clap(short, long)]
    index: bool,

    /// Do not print the sorted items on the prompt line
    #[clap(long)]
    no_inline: bool,

    /// Items that can be sorted
    items: Vec<String>,
}

impl Sort {
    pub fn run(&self) -> Result<()> {
        let item_len = self.items.len();

        if item_len == 0 {
            return Ok(());
        }

        let theme = ColorfulTheme {
            inline_selections: !self.no_inline,
            ..ColorfulTheme::default()
        };

        let mut input = dialoguer::Sort::with_theme(&theme);

        input
            .with_prompt(&self.message)
            .clear(true)
            .items(&self.items);

        let ret = if self.cancel {
            input.interact_opt()?
        } else {
            Some(input.interact()?)
        };

        let value = match ret {
            Some(value) => value,
            None if self.return_default => (0..self.items.len()).collect(),
            None => std::process::exit(1),
        };

        if self.index {
            for i in value {
                println!("{}", i);
            }
        } else {
            for i in value {
                println!("{}", self.items[i]);
            }
        }

        Ok(())
    }
}
