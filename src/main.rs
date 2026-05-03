use clap::Parser;
use colored::Colorize;
use rand::Rng;
use std::process;

use devvault::cli::{Cli, Commands, HookAction};
use devvault::vault;
use devvault::export;
use devvault::scanner;
use devvault::hooks;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("{}: {}", "error".red().bold(), e);
        process::exit(1);
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Init => {
            println!("{}", "Initializing devvault...".green());
            vault::init()?;
            println!("{}", "Vault initialized successfully!".green().bold());
        }
        Commands::Set { key_value, generate, length } => {
            let (key, value) = if generate {
                // Generate a random value
                let mut rng = rand::thread_rng();
                let random_value: String = (0..length)
                    .map(|_| {
                        let idx = rng.gen_range(0..62);
                        match idx {
                            0..=9 => (b'0' + idx) as char,
                            10..=35 => (b'a' + idx - 10) as char,
                            36..=61 => (b'A' + idx - 36) as char,
                            _ => unreachable!(),
                        }
                    })
                    .collect();
                (key_value.clone(), random_value)
            } else {
                parse_key_value(&key_value)?
            };
            vault::set(&key, &value)?;
            println!("{} {}={}", "Set".green(), key.blue(), value);
        }
        Commands::Get { key } => {
            let value = vault::get(&key)?;
            println!("{}", value);
        }
        Commands::List => {
            let vars = vault::list()?;
            if vars.is_empty() {
                println!("{}", "No variables found.".yellow());
            } else {
                for (key, entry) in vars {
                    println!("{}={}", key.blue(), entry.value);
                }
            }
        }
        Commands::Remove { key } => {
            vault::remove(&key)?;
            println!("{} {}", "Removed".green(), key.blue());
        }
        Commands::Import { file } => {
            let count = export::import_env(&file)?;
            println!("{} {} variables from {}", "Imported".green(), count, file.display());
        }
        Commands::Export { format } => {
            let content = export::export_env(&format)?;
            println!("{}", content);
        }
        Commands::Scan { path } => {
            let results = scanner::scan(path.as_deref())?;
            if results.is_empty() {
                println!("{}", "No secrets found.".green());
            } else {
                println!("{}", "Potential secrets found:".red().bold());
                for result in results {
                    println!("  {}:{} - {}", result.file.display(), result.line, result.matched.red());
                }
            }
        }
        Commands::Hook { action } => match action {
            HookAction::Install => {
                hooks::install()?;
                println!("{}", "Git pre-commit hook installed.".green());
            }
            HookAction::Uninstall => {
                hooks::uninstall()?;
                println!("{}", "Git pre-commit hook uninstalled.".green());
            }
        },
    }
    Ok(())
}

fn parse_key_value(s: &str) -> anyhow::Result<(String, String)> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid format. Expected KEY=VALUE");
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}