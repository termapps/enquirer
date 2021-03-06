use dialoguer::theme::ColorfulTheme;
use structopt::StructOpt;

use std::{io::Result, iter::repeat};

/// Prompt that allows the user to select multiple items from a list of options
#[derive(Debug, StructOpt)]
pub struct MultiSelect {
    /// Message for the prompt
    #[structopt(short, long)]
    message: String,

    /// Enables paging. Uses your terminal size
    #[structopt(short, long)]
    paged: bool,

    /// Do not print the selected items on the prompt line
    #[structopt(long)]
    no_inline: bool,

    /// Specify numbers of items that will be selected by default
    #[structopt(short, long)]
    selected: Vec<usize>,

    /// Items that can be selected
    items: Vec<String>,
}

impl MultiSelect {
    pub fn run(&self) -> Result<()> {
        let item_len = self.items.len();

        if item_len == 0 {
            return Ok(());
        }

        let theme = ColorfulTheme {
            inline_selections: !self.no_inline,
            ..ColorfulTheme::default()
        };

        let mut input = dialoguer::MultiSelect::with_theme(&theme);
        let mut defaults = vec![];

        defaults.extend(repeat(false).take(item_len));

        for i in &self.selected {
            if *i > item_len {
                continue;
            }

            defaults[i - 1] = true;
        }

        input
            .with_prompt(&self.message)
            .paged(self.paged)
            .clear(true)
            .items(&self.items)
            .defaults(&defaults);

        let value = input.interact()?;

        for i in value {
            println!("{}", self.items[i]);
        }

        Ok(())
    }
}
