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
struct Enquirer {
    #[clap(subcommand)]
    cmd: EnquirerSubcommand,

    /// Disable colors in the prompt
    #[clap(long)]
    no_color: bool,
}

#[derive(Debug, Parser)]
enum EnquirerSubcommand {
    Confirm(confirm::Confirm),
    Input(input::Input),
    Secret(secret::Secret),
    MultiSelect(multi_select::MultiSelect),
    Select(select::Select),
    Sort(sort::Sort),
}

fn main() {
    // TODO: Specify height for selection prompts (like fzf)
    let program = Enquirer::parse();
    set_colors_enabled(!program.no_color);

    match program.cmd {
        EnquirerSubcommand::Confirm(x) => x.run(),
        EnquirerSubcommand::Input(x) => x.run(),
        EnquirerSubcommand::Secret(x) => x.run(),
        EnquirerSubcommand::MultiSelect(x) => x.run(),
        EnquirerSubcommand::Select(x) => x.run(),
        EnquirerSubcommand::Sort(x) => x.run(),
    }
    .unwrap();
}
