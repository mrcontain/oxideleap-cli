use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct SetupOption {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(
        short,
        long,
        value_name = "FILE",
        default_value = "/etc/kickserver/config.yaml"
    )]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}
