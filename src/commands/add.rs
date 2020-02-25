use argh::FromArgs;

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

pub fn execute(options: AddOptions) {
    println!(
        "{} + {} = {}",
        options.num1,
        options.num2,
        options.num1 + options.num2
    );
}
