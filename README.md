# Argh Demo

> Real knowledge comes from practice.

Recently, I noticed a command line parameter parsing tool called "argh", which is lightweight, easy to use, and really human friendly.

I like it for two reasons:

- Small amount of code, simple and intuitive
- Ingenious use of "Rust" syntax

Here I will talk about the use of "argh" from the perspective of the user, as a small "prompt board" for myself.

## Command Line Args

First, let's create a new project. Naturally, `cargo new <argh-demo>`.

Then add argh to our dependencies like this:

```toml
[dependencies]
argh = "0.1"
```

Let's do something together, such as implementing a simple command line adder? It's like "a + b".
Obviously, our function takes two arguments, `num1` and `num2`.

```rust
fn add(num1: u16, num2: u16) {
    println!(arg1 + arg2);
}
```

See how _argh_ does it:

```rust
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
```

We then connected them together in the main function.

```rust
fn main() {
    let cli: DemoCli = argh::from_env();
    add(cli.num1, cli.num2);
}
```

Now, let's try to run `cargo run` and see what happens:

```text
Required options not provided:
    --num1
    --num2
```

Try the `--help` option,`target / debug / argh-demo --help`:

```text
Usage: target/debug/argh-demo --num1 <num1> --num2 <num2>

Add two numbers

Options:
  --num1            the first number.
  --num2            the second number
  --help            display usage information
```

Well, it seems we have to add all the options `target/debug/argh-demo --num1 1 --num2 2`：

```text
1 + 2 = 3
```

## Subcommands

It looks good, but if our goal is a complete calculator, select functions by subcommands, such as `argh-demo add --num1 1-num2 2`,
This will make some changes in our design.

Let's see how to change the code:

1. First declare a set of subcommands named `subcommand` in `DemoCli`.

   ```rust
   #[derive(FromArgs)]
   /// A simple calculation tool
   struct DemoCli {
       #[argh(subcommand)]
       subcommand: SubCommands,
   }
   ```

2. Define the structure `SubCommands` containing the `Add` option:

   ```rust
   #[derive(FromArgs, PartialEq, Debug)]
   #[argh(subcommand)]
   enum SubCommands {
       Add(AddOptions),
   }
   ```

3. Add content for `AddOptions`:

   ```rust
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
   ```

Obviously, you just need to rewrite the calling part in the main function:

```rust
match cli.subcommand {
    SubCommands::Add(options) => {
        add(options.num1, options.num2);
    }
};
```

Run `cargo build && ./target/debug/argh-demo --help` to see how we can use this improved version.

```text
Usage: ./target/debug/argh-demo <command> [<args>]

A simple calculation tool

Options:
  --help            display usage information

Commands:
  add               Add two numbers
```

Continue to use `1 + 2 = 3` to try, `target / debug / argh-demo --num1 1 --num2 2`:

```text
1 + 2 = 3
```

### Better Structure

If our program has a lot of options / subcommands to organize, it seems messy to put in a single file.
Imagine how programs like `cargo` do this. They have separate files for each subcommand in the `commands` folder.

If we were to introduce a `sub` subcommand, the program structure would look like this.

```text
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   ├── commands
│   │   ├── add.rs
│   │   ├── mod.rs
│   │   └── sub.rs
│   └── main.rs
```

Since both the subcommands and the corresponding `execute` functions should be moved to the corresponding files in `commands`,
`main.rs` will be much streamlined:

```rust
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
```

So obviously, the `add` and `sub` modules should be exposed in `commands mod.rs`.

```rust
pub mod add;
pub mod sub;
```

Since we pass the options as a whole to the `execute` function, the `add` module needs to be modified accordingly:

```rust
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
```

The next step is nothing more than writing a simple `sub` module:

```rust
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
```

As usual, use `--help` option to see usage:

```text
Usage: target/debug/argh-demo <command> [<args>]

A simple calculation tool

Options:
  --help            display usage information

Commands:
  add               Add two numbers
  sub               Sub two numbers
```

Finally test the functions again:

1. `target/debug/argh-demo add --num1 1 --num2 2`：

   ```text
   1 + 2 = 3
   ```

2. `target/debug/argh-demo sub --num1 1 --num2 2`：

   ```text
   1 - 2 = -1
   ```
