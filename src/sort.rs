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

        let value = input.interact()?;

        for i in value {
            println!("{}", self.items[i]);
        }

        Ok(())
    }
}
