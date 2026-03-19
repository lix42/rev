mod app;
mod config;
mod diff;
mod git;
mod input;
mod review;
mod ui;
mod watcher;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "rev",
    about = "TUI-native code review tool for AI agent workflows"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Export review comments
    Export {
        /// Output format: text, markdown, json
        #[arg(long, default_value = "text")]
        format: String,
        /// Filter by status: open, resolved, updated, all
        #[arg(long, default_value = "open")]
        status: String,
        /// Filter by file path
        #[arg(long)]
        file: Option<String>,
    },
    /// Close a review session
    Close,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // TODO: pass _repo_info to subcommands once they consume it
    let _repo_info = match git::open_repo(&std::env::current_dir()?) {
        Ok(info) => info,
        Err(e) => {
            eprintln!(
                "rev: {e:#}\n\n\
                 rev currently requires a git repo with at least one commit.\n\
                 Run it from inside a project that uses git."
            );
            // No resources to clean up at this point; exit before the TUI starts.
            std::process::exit(1);
        }
    };

    match cli.command {
        Some(Command::Export {
            format,
            status,
            file,
        }) => review::export::run(&format, &status, file.as_deref()),
        Some(Command::Close) => review::session::close(),
        None => app::run().await,
    }
}
