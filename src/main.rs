use clap::{Parser, Subcommand};
use git2::{Repository, StatusOptions};
use std::time::{SystemTime, UNIX_EPOCH};

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
    /// Show git working tree status
    Status {
        /// Path to the repository (default: current directory)
        path: Option<String>,
    },
    /// Show recent commit logs
    Log {
        /// Path to the repository (default: current directory)
        path: Option<String>,
        /// Number of commits to show
        #[arg(short, long, default_value = "10")]
        n: usize,
    },
    /// List all branches
    Branch {
        /// Path to the repository (default: current directory)
        path: Option<String>,
    },
    /// Show current branch name
    BranchName {
        /// Path to the repository (default: current directory)
        path: Option<String>,
    },
}

fn get_repo(path: Option<&str>) -> Result<Repository, String> {
    let repo_path = if let Some(p) = path {
        std::path::Path::new(p).to_path_buf()
    } else {
        std::env::current_dir().map_err(|e| e.to_string())?
    };
    Repository::open(&repo_path).map_err(|e| format!("Failed to open repository: {}", e))
}

fn format_time(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    if days > 0 {
        format!("{}d ago", days)
    } else if hours > 0 {
        format!("{}h ago", hours)
    } else if minutes > 0 {
        format!("{}m ago", minutes)
    } else {
        "just now".to_string()
    }
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
        Command::Status { path } => {
            if let Ok(repo) = get_repo(path.as_deref()) {
                let mut opts = StatusOptions::new();
                opts.include_untracked(true);
                let statuses = repo.statuses(Some(&mut opts));
                match statuses {
                    Ok(statuses) => {
                        if statuses.is_empty() {
                            println!("Nothing to commit, working tree clean");
                        } else {
                            for entry in statuses.iter() {
                                let status = entry.status();
                                let path = entry.path().unwrap_or("?");
                                let mut s = String::new();
                                if status.is_index_new() { s.push('A'); }
                                else if status.is_index_modified() { s.push('M'); }
                                else if status.is_index_deleted() { s.push('D'); }
                                else if status.is_index_renamed() { s.push('R'); }
                                else if status.is_wt_new() { s.push('?'); }
                                else if status.is_wt_modified() { s.push('M'); }
                                else if status.is_wt_deleted() { s.push('D'); }
                                else { s.push('?'); }
                                println!("{} {}", s, path);
                            }
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else if let Err(e) = get_repo(path.as_deref()) {
                eprintln!("Error: {}", e);
            }
        }
        Command::Log { path, n } => {
            if let Ok(repo) = get_repo(path.as_deref()) {
                let mut revwalk = match repo.revwalk() {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        return;
                    }
                };
                let _ = revwalk.push_head();
                for (i, oid) in revwalk.enumerate() {
                    if i >= n { break; }
                    if let Ok(oid) = oid {
                        if let Ok(commit) = repo.find_commit(oid) {
                            let author_name = commit.author().name().unwrap_or("Unknown").to_string();
                            let msg = commit.message().unwrap_or("").lines().next().unwrap_or("");
                            let time = commit.time().seconds();
                            let sys_time = UNIX_EPOCH + std::time::Duration::from_secs(time as u64);
                            println!("{} {} ({})", &oid.to_string()[..7], msg, author_name);
                            println!("    {}", format_time(sys_time));
                        }
                    }
                }
            } else if let Err(e) = get_repo(path.as_deref()) {
                eprintln!("Error: {}", e);
            }
        }
        Command::Branch { path } => {
            if let Ok(repo) = get_repo(path.as_deref()) {
                let head = repo.head().ok();
                let head_oid = head.as_ref().and_then(|h| h.target());
                for branch in repo.branches(Some(git2::BranchType::Local)).unwrap().filter_map(|b| b.ok()) {
                    let name = branch.0.name().unwrap_or(Some("?")).unwrap_or("?");
                    let is_current = head_oid == branch.0.get().target();
                    if is_current {
                        println!("* {}", name);
                    } else {
                        println!("  {}", name);
                    }
                }
            } else if let Err(e) = get_repo(path.as_deref()) {
                eprintln!("Error: {}", e);
            }
        }
        Command::BranchName { path } => {
            if let Ok(repo) = get_repo(path.as_deref()) {
                match repo.head() {
                    Ok(head) => {
                        if let Some(name) = head.shorthand() {
                            println!("{}", name);
                        } else {
                            eprintln!("Error: HEAD is detached");
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else if let Err(e) = get_repo(path.as_deref()) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
