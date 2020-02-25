//! Just a demo for argh.

use argh::FromArgs;

#[derive(FromArgs)]
/// A simple calculation tool
struct DemoCli {
    #[argh(subcommand)]
    subcommand: SubCommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommands {
    Add(AddOptions),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Add two numbers
#[argh(subcommand, name = "add")]
pub struct AddOptions {
    /// the first number.
    #[argh(option)]
    num1: u16,

    /// the second number
    #[argh(option)]
    num2: u16,
}

fn main() {
    let cli: DemoCli = argh::from_env();
    match cli.subcommand {
        SubCommands::Add(options) => {
            add(options.num1, options.num2);
        }
    };
}

fn add(num1: u16, num2: u16) {
    println!("{} + {} = {}", num1, num2, num1 + num2);
}
