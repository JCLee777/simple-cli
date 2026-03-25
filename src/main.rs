use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "simple-cli",
    version,
    about = "A tiny Rust CLI example packaged with cargo-dist"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Greet someone by name
    Hello {
        /// The name to greet
        #[arg(short, long, default_value = "world")]
        name: String,
    },
    /// Add two integers together
    Sum {
        /// The left-hand value
        left: i64,
        /// The right-hand value
        right: i64,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Hello { name } => {
            println!("Hello, {name}!");
        }
        Command::Sum { left, right } => {
            println!("{}", left + right);
        }
    }
}
