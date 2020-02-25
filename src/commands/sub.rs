use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Sub two numbers
#[argh(subcommand, name = "sub")]
pub struct SubOptions {
    /// the first number.
    #[argh(option)]
    num1: i16,

    /// the second number
    #[argh(option)]
    num2: i16,
}

pub fn execute(options: SubOptions) {
    println!(
        "{} - {} = {}",
        options.num1,
        options.num2,
        options.num1 - options.num2
    );
}
