mod confirm;
mod input;
mod multi_select;
mod secret;
mod select;
mod sort;

use clap::Parser;
use dialoguer::console::set_colors_enabled;

/// Command line utility for stylish interactive prompts
#[derive(Debug, Parser)]
#[clap(name = "enquirer", version)]
struct App {
    #[clap(subcommand)]
    cmd: Subcommands,

    /// Disable colors in the prompt
    #[clap(long)]
    no_color: bool,
}

#[derive(Debug, Parser)]
enum Subcommands {
    Confirm(confirm::Confirm),
    Input(input::Input),
    Secret(secret::Secret),
    MultiSelect(multi_select::MultiSelect),
    Select(select::Select),
    Sort(sort::Sort),
}

fn main() {
    // TODO: Specify height for selection prompts (like fzf)
    let program = App::parse();
    set_colors_enabled(!program.no_color);

    match program.cmd {
        Subcommands::Confirm(x) => x.run(),
        Subcommands::Input(x) => x.run(),
        Subcommands::Secret(x) => x.run(),
        Subcommands::MultiSelect(x) => x.run(),
        Subcommands::Select(x) => x.run(),
        Subcommands::Sort(x) => x.run(),
    }
    .unwrap();
}
