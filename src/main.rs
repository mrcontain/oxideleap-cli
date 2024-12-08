use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
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
    /// input the username
    #[arg(short, long, value_name = "USERNAME")]
    user: String,

    /// input the password
    #[arg(short, long)]
    password: String,

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
            let cli_header = format!(
                "{}{}{}{}{}",
                "kickserver".blue(),
                "(".yellow(),
                args.user.purple(),
                ")".yellow(),
                "=>".green()
            );
            let mut stdout = io::stdout();
            // todo: following need to query the database and verify the username and password
            // let password = args.password;
            loop {
                print_with_anyoutput(&cli_header, &mut stdout)?;
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

fn print_with_anyoutput<T: Write>(cli_header: &str, output: &mut T) -> Result<()> {
    output.write_all(cli_header.as_bytes())?;
    output.flush()?;
    Ok(())
}
