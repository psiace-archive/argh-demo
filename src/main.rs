use argh::FromArgs;

#[derive(FromArgs)]
/// Add two numbers
struct DemoCli {
    /// the first number.
    #[argh(option)]
    num1: u16,

    /// the second number
    #[argh(option)]
    num2: u16,
}

fn main() {
    let cli: DemoCli = argh::from_env();
    add(cli.num1, cli.num2);
}

fn add(num1: u16, num2: u16) {
    println!("{} + {} = {}", num1, num2, num1 + num2);
}
