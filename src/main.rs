mod confirm;
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
}

fn main() {
    let program = Enquirer::from_args();

    set_colors_enabled(!program.no_color);

    match program.cmd {
        EnquirerSubcommand::Confirm(x) => x.run(),
    }
    .unwrap();
}
