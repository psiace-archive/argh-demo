# Argh - 人类友好的命令行参数解析

> 实践出真知。

最近，我注意到一个名为「argh」的命令行参数解析工具，它很轻巧、易用，而且确实对人类友好。

我喜欢它有以下两点原因：

- 代码量小，而且简洁、直观
- 巧妙地利用「Rust」的语法

这里我将会从用户的角度来讲一下「argh」的使用，就当作自己的一个小「提词板」。

## 命令行参数

首先，我们新建一个项目，很自然地 `cargo new <argh-demo>`。

然后将 argh 添加到我们的依赖项中，像下面这样：

```toml
[dependencies]
argh = "0.1"
```

让我们一起做点什么，比如实现一个简单的命令行加法程序？就像 `a + b` 那个样子。
很显然，我们的函数需要两个参数 `arg1` 和 `arg2`。

```rust
fn add(num1: u16, num2: u16) {
    println!(arg1 + arg2);
}
```

看看 argh 是怎么做的：

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

然后，我们在 `main` 函数中将它们连在一起。

```rust
fn main() {
    let cli: DemoCli = argh::from_env();
    add(cli.num1, cli.num2);
}
```

现在，让我们试着运行一下 `cargo run`，看看会发生什么：

```text
Required options not provided:
    --num1
    --num2
```

试试 `--help` 选项，`target/debug/argh-demo --help`：

```text
Usage: target/debug/argh-demo --num1 <num1> --num2 <num2>

Add two numbers

Options:
  --num1            the first number.
  --num2            the second number
  --help            display usage information
```

好吧，看来我们不得不加上参数 `target/debug/argh-demo --num1 1 --num2 2`：

```text
1 + 2 = 3
```

## 子命令

看起来还不错，但如果我们的目标是一个完整的计算器，通过子命令来选择功能，如 `argh-demo add --num1 1 -- num2 2`，
这会让我们的设计发生一些变化。

让我们看一下怎么更改代码：

1. 先在 `DemoCli` 中声明一组名为 `subcommand` 的子命令。

   ```rust
   #[derive(FromArgs)]
   /// A simple calculation tool
   struct DemoCli {
       #[argh(subcommand)]
       subcommand: SubCommands,
   }
   ```

2. 定义包含 `Add` 选项的结构体 `SubCommands`:

   ```rust
   #[derive(FromArgs, PartialEq, Debug)]
   #[argh(subcommand)]
   enum SubCommands {
       Add(AddOptions),
   }
   ```

3. 为 `AddOptions` 添加内容：

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

很显然，接下来就只需要在主函数中重写调用部分即可：

```rust
match cli.subcommand {
    SubCommands::Add(options) => {
        add(options.num1, options.num2);
    }
};
```

运行 `cargo build && ./target/debug/argh-demo --help` 看看我们该怎么使用这个改进后的版本。

```text
Usage: ./target/debug/argh-demo <command> [<args>]

A simple calculation tool

Options:
  --help            display usage information

Commands:
  add               Add two numbers
```

继续用 `1 + 2 = 3` 试试看，`target/debug/argh-demo --num1 1 --num2 2`：

```text
1 + 2 = 3
```

### 更好的程序结构

如果我们的程序有很多选项/子命令需要组织，放在单一文件里似乎会显得很乱。
想想像 `cargo` 这样的程序是如何做的，它们在 `commands` 文件夹中为每个子命令准备了单独的文件。

如果我们要引入一个 `sub` 命令，那么程序结构应该像下面这样。

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

由于子命令和对应的执行函数都应该被移动到 `commands` 中对应的文件里，所以 `main.rs` 会精简许多：

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

那么很显然，`commands/mod.rs` 中应该将 `add` 和 `sub` 模块公开。

```rust
pub mod add;
pub mod sub;
```

由于我们将参数作为整体传入执行函数，所以 `add` 模块也需要对应修改：

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

接下来，无非就是照猫画虎写一个简单的 `sub` 模块：

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

老规矩，用 `--help` 选项查看用法：

```text
Usage: target/debug/argh-demo <command> [<args>]

A simple calculation tool

Options:
  --help            display usage information

Commands:
  add               Add two numbers
  sub               Sub two numbers
```

最后再测试一下功能：

1. `target/debug/argh-demo add --num1 1 --num2 2`：

   ```text
   1 + 2 = 3
   ```

2. `target/debug/argh-demo sub --num1 1 --num2 2`：

   ```text
   1 - 2 = -1
   ```
