use dialoguer::theme::ColorfulTheme;
use structopt::StructOpt;

use std::io::Result;

/// Prompt that allows the user to sort items in a list
#[derive(Debug, StructOpt)]
pub struct Sort {
    /// Message for the prompt
    #[structopt(short, long)]
    message: String,

    /// Enables paging. Uses your terminal size
    #[structopt(short, long)]
    paged: bool,

    /// Makes the prompt cancellable with 'Esc' or 'q'.
    #[structopt(short, long)]
    cancel: bool,

    /// Makes the prompt return default order as given if --cancel option is present.
    #[structopt(short = "d", long = "default")]
    return_default: bool,

    /// Returns index of the sorted items instead of items itself
    #[structopt(short, long)]
    index: bool,

    /// Do not print the sorted items on the prompt line
    #[structopt(long)]
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
            .paged(self.paged)
            .clear(true)
            .items(&self.items);

        let ret = if self.cancel {
            input.interact_opt()?
        } else {
            input.interact().ok()
        };

        let value = match ret {
            Some(value) => value,
            None if self.return_default => 0..self.items.len(),
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
