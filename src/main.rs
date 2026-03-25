use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(
    name = "simple-cli",
    version,
    about = "A tiny Rust CLI example packaged with cargo-dist"
)]
struct Cli {
    #[command(subcommand)]
    command: GitCommand,
}

#[derive(Subcommand, Debug)]
enum GitCommand {
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
    /// Show git working tree status
    Status {
        /// Path to the repository (default: current directory)
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Show recent commit logs
    Log {
        /// Path to the repository (default: current directory)
        #[arg(short, long)]
        path: Option<String>,
        /// Number of commits to show
        #[arg(short, long, default_value = "10")]
        n: usize,
    },
    /// List all branches
    Branch {
        /// Path to the repository (default: current directory)
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Show current branch name
    BranchName {
        /// Path to the repository (default: current directory)
        #[arg(short, long)]
        path: Option<String>,
    },
}

fn run_git(args: &[&str], path: Option<&str>) -> std::process::Output {
    let mut cmd = Command::new("git");
    cmd.args(args);
    if let Some(p) = path {
        cmd.current_dir(p);
    }
    cmd.output().expect("failed to execute git")
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        GitCommand::Hello { name } => {
            println!("Hello, {name}!");
        }
        GitCommand::Sum { left, right } => {
            println!("{}", left + right);
        }
        GitCommand::Status { path } => {
            let output = run_git(&["status", "--porcelain"], path.as_deref());
            if output.stdout.is_empty() {
                println!("Nothing to commit, working tree clean");
            } else {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
        GitCommand::Log { path, n } => {
            let output = run_git(&["log", &format!("-{}", n), "--oneline"], path.as_deref());
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        GitCommand::Branch { path } => {
            let output = run_git(&["branch", "-a"], path.as_deref());
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        GitCommand::BranchName { path } => {
            let output = run_git(&["branch", "--show-current"], path.as_deref());
            let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if name.is_empty() {
                eprintln!("Error: HEAD is detached");
            } else {
                println!("{}", name);
            }
        }
    }
}
