use anyhow::Result;
use clap::{Parser, Subcommand};
use kick_server_cli::process;
use kick_server_cli::setup_option::SetupOption;
use std::io::{self, Write};
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Kick Server CLI",
    long_about = "specialize in connecting and managing kickserver"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Initialize a new kickserver by config.yaml
    #[command(version, about = "Setup a new kickserver")]
    Setup(SetupOption),
}
fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Some(SubCommand::Setup(setup)) => {
            let values = process::process_yaml(setup.file)?;
            for value in values {
                println!("{:?}", value);
            }
            todo!()
        }
        None => {
            let mut stdout = io::stdout();
            loop {
                print!("kkcli=>");
                stdout.flush()?;
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                let input = input.trim();
                if input == "exit" {
                    break;
                }
            }
        }
    }
    Ok(())
}
