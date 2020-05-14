use dialoguer::theme::ColorfulTheme;
use std::io::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Prompt that allows the user to select from a list of options
pub struct Select {
    #[structopt(short, long)]
    /// Message for the prompt
    message: String,

    /// Enables paging. Uses your terminal size
    #[structopt(short, long)]
    paged: bool,

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

        let value = input.interact()?;

        println!("{}", self.items[value]);

        Ok(())
    }
}
