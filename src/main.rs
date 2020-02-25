//! Just a demo for argh.

use argh::FromArgs;

mod commands;

#[derive(FromArgs)]
/// A simple calculation tool
struct DemoCli {
    #[argh(subcommand)]
    subcommand: SubCommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommands {
    Add(commands::add::AddOptions),
    Sub(commands::sub::SubOptions),
}

fn main() {
    let cli: DemoCli = argh::from_env();
    match cli.subcommand {
        SubCommands::Add(options) => {
            commands::add::execute(options);
        }
        SubCommands::Sub(options) => {
            commands::sub::execute(options);
        }
    };
}
