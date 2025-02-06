/*
 * @Author: mrcontain 1916985079@qq.com
 * @Date: 2024-12-07 12:40:35
 * @LastEditors: mrcontain 1916985079@qq.com
 * @LastEditTime: 2025-02-06 10:26:02
 * @FilePath: \oxideleap-cli\src\main.rs
 * @Description:
 *
 * Copyright (c) 2024 by ${git_name_email}, All Rights Reserved.
 */
use anyhow::Result;
use clap::Parser;
use colored::*;
use oxideleap_cli::process::process_yaml;
use oxideleap_cli::{print, process};
use oxideleap_cli::{Cli, SubCommand};
use std::io;
use std::path::PathBuf;
use std::process::exit;

const USER_CONFIG_PATH: &str = "assets/test.yaml";
// const USER_CONFIG_PATH: &str = "/etc/oxideleap-client/config.yaml";

fn main() -> Result<()> {
    let mut args = Cli::parse();
    // judge username and password whether already input
    if args.user.is_none() && args.password.is_none() {
        let values = process_yaml(PathBuf::from(USER_CONFIG_PATH))?;
        args.user = values[0]["username"]
            .as_str()
            .map(|s| s.to_string())
            .or_else(|| {
                eprintln!("username is not set in the config file");
                exit(1);
            });
        args.password = values[0]["password"]
            .as_str()
            .map(|s| s.to_string())
            .or_else(|| {
                eprintln!("password is not set in the config file");
                exit(1);
            });
    }
    match args.command {
        Some(SubCommand::Setup(setup)) => {
            let values = process::process_yaml(setup.file)?;
            for value in values {
                println!("{:?}", value);
            }
            todo!()
        }
        Some(_) => {
            // Handle the Connect command
            println!("invalid arguments");
        }
        None => {
            let cli_header = format!(
                "{}{}{}{}{}",
                "OL-cli".blue(),
                "(".yellow(),
                args.user.unwrap().purple(),
                ")".yellow(),
                "=>".green()
            );
            let mut stdout = io::stdout();
            // TODO: following need to query the database and verify the username and password
            // let password = args.password;
            loop {
                print::print_with_anyoutput(&cli_header, &mut stdout)?;
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                let input = input.trim();
                if input == "exit" {
                    break;
                }

                // 解析用户输入的命令
                let input_args = format!("oxideleap-cli {}", input);
                let input_cli = Cli::try_parse_from(input_args.split_whitespace());

                match input_cli {
                    Ok(cli) => match cli.command {
                        Some(SubCommand::Setup { .. }) => {
                            println!("The 'setup' command is not allowed in interactive mode.");
                        }
                        Some(_) => {
                            println!("Unknown command");
                        }
                        None => {
                            // do nothing
                        }
                    },
                    Err(err) => {
                        eprintln!("{}", err);
                    }
                }
            }
        }
    }
    Ok(())
}
