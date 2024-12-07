use anyhow::{Error as AnyError, Ok};
use clap::{Parser, Subcommand};
use kick_server_cli::process;
use kick_server_cli::setup_option::SetupOption;
use std::path::PathBuf;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[command(version, about, long_about = None)]
    Setup(SetupOption),
}
fn main() -> Result<(), AnyError> {
    let args = Cli::parse();
    match args.command {
        Some(SubCommand::Setup(setup)) => {
            let values = process::process_yaml(setup.file)?;
            for value in values {
                println!("{:?}", value);
            }
        }
        None => {
            println!("No subcommand");
        }
    }
    Ok(())
}
