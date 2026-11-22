use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(
    name = "audit-triage",
    version,
    about = "Triage and report Anchor lint findings"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a path, update state, and print a summary
    Scan {
        /// Path to source code or directory to scan
        path: PathBuf,
        /// Path to state file
        #[arg(long)]
        state: Option<PathBuf>,
    },
    /// Generate a report for a scanned path
    Report {
        /// Path to source code or directory
        path: PathBuf,
        /// Path to state file
        #[arg(long)]
        state: Option<PathBuf>,
        /// Output file path
        #[arg(long)]
        out: Option<PathBuf>,
        /// Report format (md or json)
        #[arg(long)]
        format: Option<String>,
    },
    /// Mark finding status by fingerprint prefix
    Mark {
        /// Fingerprint prefix to match
        fingerprint_prefix: String,
        /// New status for the finding
        status: String,
        /// Optional note
        #[arg(long)]
        note: Option<String>,
        /// Path to state file
        #[arg(long)]
        state: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan { .. } | Commands::Report { .. } | Commands::Mark { .. } => {
            eprintln!("not implemented yet (milestone M2-M4)");
            process::exit(2);
        }
    }
}
