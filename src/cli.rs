use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "devvault", about = "Developer secrets & .env manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new vault in the current directory
    Init,
    /// Set an environment variable (KEY=VALUE or KEY with --generate)
    Set {
        /// Variable in KEY=VALUE format or just KEY with --generate
        key_value: String,
        /// Generate a random value for the key
        #[arg(long, short)]
        generate: bool,
        /// Length of generated random value (default: 32)
        #[arg(long, default_value = "32")]
        length: usize,
    },
    /// Get an environment variable value
    Get {
        /// Variable name
        key: String,
    },
    /// List all environment variables
    List,
    /// Remove an environment variable
    Remove {
        /// Variable name to remove
        key: String,
    },
    /// Import variables from a .env file
    Import {
        /// Path to .env file
        file: PathBuf,
    },
    /// Export variables to .env format
    Export {
        /// Output format (default: env)
        #[arg(long, default_value = "env")]
        format: String,
    },
    /// Scan for hardcoded secrets
    Scan {
        /// Path to scan (default: current directory)
        path: Option<PathBuf>,
    },
    /// Manage git hooks
    Hook {
        #[command(subcommand)]
        action: HookAction,
    },
}

#[derive(Subcommand)]
pub enum HookAction {
    /// Install git pre-commit hook
    Install,
    /// Uninstall git pre-commit hook
    Uninstall,
}