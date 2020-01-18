mod confirm;
mod input;
mod multi_select;
mod secret;
mod theme;

use console::set_colors_enabled;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "enquirer")]
/// Command Line Utility for Stylish Interactive Prompts
struct Enquirer {
    #[structopt(subcommand)]
    cmd: EnquirerSubcommand,

    #[structopt(long)]
    /// Disable colors in the prompt
    no_color: bool,
}

#[derive(Debug, StructOpt)]
enum EnquirerSubcommand {
    Confirm(confirm::Confirm),
    Input(input::Input),
    Secret(secret::Secret),
    MultiSelect(multi_select::MultiSelect),
}

fn main() {
    // TODO: Specify height for selection prompts (like fzf)
    let program = Enquirer::from_args();
    set_colors_enabled(!program.no_color);

    match program.cmd {
        EnquirerSubcommand::Confirm(x) => x.run(),
        EnquirerSubcommand::Input(x) => x.run(),
        EnquirerSubcommand::Secret(x) => x.run(),
        EnquirerSubcommand::MultiSelect(x) => x.run(),
    }
    .unwrap();
}
