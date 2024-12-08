use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct SetupOption {
    /// Sets a custom config file
    #[arg(
        short,
        long,
        value_name = "INPUT_FILE",
        default_value = "/etc/kickserver/config.yaml"
    )]
    pub file: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}
