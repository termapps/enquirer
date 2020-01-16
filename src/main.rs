mod confirm;
mod theme;

use console::set_colors_enabled;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "enquirer")]
/// Command Line Utility for Interactive Prompts
enum Enquirer {
    Confirm(confirm::Confirm),
}

fn main() {
    set_colors_enabled(true);

    match Enquirer::from_args() {
        Enquirer::Confirm(x) => x.run(),
    }
    .unwrap();
}
