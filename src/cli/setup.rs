/*
 * @Author: mrcontain 1916985079@qq.com
 * @Date: 2024-12-07 16:10:06
 * @LastEditors: mrcontain 1916985079@qq.com
 * @LastEditTime: 2025-01-09 09:13:47
 * @FilePath: \oxideleap-cli\src\cli\setup.rs
 * @Description:
 *
 * Copyright (c) 2025 by ${git_name_email}, All Rights Reserved.
 */
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
