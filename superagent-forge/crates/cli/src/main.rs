use clap::{Parser, Subcommand};
use std::{error::Error, path::PathBuf, process::ExitCode};

#[derive(Parser)]
#[command(
    name = "forge",
    version,
    about = "Superagent Forge architecture foundation (execution not enabled)"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate configuration. Does not create directories, open a database or start workers.
    Check {
        #[arg(short, long, default_value = "config/default.toml")]
        config: PathBuf,
    },
    /// Print reserved MCP tool contracts. Does not start an MCP server.
    Protocol,
}

fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    match cli.command {
        Command::Check { config } => {
            let config = forge_config::Config::parse(&std::fs::read_to_string(config)?)?;
            println!(
                "configuration valid; concurrency={}; network_allowed={}; worker execution and SQLite backend are not implemented",
                config.max_concurrency, config.network_allowed
            );
        }
        Command::Protocol => println!(
            "{}",
            serde_json::to_string_pretty(&forge_mcp_server::planned_tools())?
        ),
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("forge: {error}");
            ExitCode::FAILURE
        }
    }
}
