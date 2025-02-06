/*
 * @Author: mrcontain 1916985079@qq.com
 * @Date: 2024-12-31 15:38:00
 * @LastEditors: mrcontain 1916985079@qq.com
 * @LastEditTime: 2025-01-14 16:26:22
 * @FilePath: \oxideleap-cli\src\cli.rs
 * @Description:
 *
 * Copyright (c) 2025 by ${git_name_email}, All Rights Reserved.
 */
mod connect;
mod setup;
use clap::{Parser, Subcommand};
use connect::*;
use setup::SetupOption;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Oxide Leap CLI",
    long_about = "specialize in connecting and managing kickserver"
)]
pub struct Cli {
    /// input the username
    #[arg(short, long, value_name = "USERNAME")]
    pub user: Option<String>,

    /// input the password
    #[arg(short, long)]
    pub password: Option<String>,

    #[command(subcommand)]
    pub command: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Initialize a new kickserver by config.yaml
    #[command(version, about = "Setup a new kickserver")]
    Setup(SetupOption),

    /// connect the remote host by ipv4 address
    #[command(version, about = "Connect to the remote host")]
    Connect(ConnectOption),
}
