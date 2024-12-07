use clap::{Parser, Subcommand};
use kick_server_cli::setup_option::SetupOption;
use std::path::PathBuf;
#[derive(Parser)]
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

#[derive(Subcommand)]
enum SubCommand {
    #[command(version, about, long_about = None)]
    Setup(SetupOption),
}
fn main() {
    println!("Hello, world!");
}
