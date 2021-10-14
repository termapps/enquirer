use dialoguer::theme::ColorfulTheme;
use structopt::StructOpt;

use std::io::Result;

/// Prompt that allows the user to select from a list of options
#[derive(Debug, StructOpt)]
pub struct Select {
    /// Message for the prompt
    #[structopt(short, long)]
    message: String,

    /// Enables paging. Uses your terminal size
    #[structopt(short, long)]
    paged: bool,

    /// Makes the prompt cancellable with 'Esc' or 'q'.
    #[structopt(short, long)]
    cancel: bool,

    /// Returns index of the selected item instead of item itself
    #[structopt(short, long)]
    index: bool,

    /// Specify number of the item that will be selected by default
    #[structopt(short, long)]
    selected: Option<usize>,

    /// Items that can be selected
    items: Vec<String>,
}

impl Select {
    pub fn run(&self) -> Result<()> {
        let item_len = self.items.len();

        if item_len == 0 {
            return Ok(());
        }

        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Select::with_theme(&theme);

        input
            .with_prompt(&self.message)
            .paged(self.paged)
            .clear(true)
            .items(&self.items);

        if self.selected.is_some() {
            input.default(self.selected.unwrap() - 1);
        }

        let ret = if self.cancel {
            input.interact_opt()?
        } else {
            input.interact().ok()
        };

        let value = match ret {
            Some(value) => value,
            None => std::process::exit(1),
        };

        if self.index {
            println!("{}", value);
        } else {
            println!("{}", self.items[value]);
        }

        Ok(())
    }
}
