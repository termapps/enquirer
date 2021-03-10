use clap::Parser;

/// A simple CLI application using clap
#[derive(Debug, Parser)]
#[clap(name = "cli-clap")]
struct App {
    #[clap(subcommand)]
    cmd: Subcommands,
}

#[derive(Debug, Parser)]
enum Subcommands {}

fn main() {
    let program = App::parse();

    match program.cmd {
        // Subcommands::Pie(x) => x.run(),
    }
}
