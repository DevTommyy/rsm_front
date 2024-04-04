// NOTE: same thing as before but builder pattern
// https://docs.rs/clap/latest/clap/_tutorial/chapter_0/index.html

use crate::error::Result;
use clap::{command, Arg, ArgAction, Command};
use utils::config_helper::Config;

pub mod api;
pub mod error;
pub mod utils;

fn app_args() -> clap::ArgMatches {
    command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("new-key").about("Resets the account key")) //TODO: confirmation
        .subcommand(
            Command::new("create")
                .about("Creates a new table")
                .arg(
                    Arg::new("tablename")
                        .required(true)
                        .help("Name of the table to create"),
                )
                .arg(
                    Arg::new("due")
                        .long("due")
                        .short('d')
                        .action(ArgAction::SetTrue)
                        .help("Set if the table has due time, defaults to false"),
                ),
        )
        .subcommand(
            Command::new("delete").about("Deletes a table").arg(
                Arg::new("tablename")
                    .required(true)
                    .help("Name of the table to remove"),
            ),
        )
        .subcommand(
            Command::new("list")
                .about("List tables with specs or table contents")
                .arg(
                    Arg::new("tablename")
                        .required(false)
                        .help("Name of the table to show"),
                )
                .arg(
                    Arg::new("group")
                        .short('g')
                        .long("group")
                        .requires("tablename")
                        .help("Specify the group to show"),
                )
                .arg(
                    Arg::new("sort-by")
                        .short('s')
                        .long("sort-by")
                        .requires("tablename")
                        .help("The key to sort the output by"), // .value_parser(["due", "group"]),
                ),
        )
        .get_matches()
}

fn main() -> Result<()> {
    log4rs::init_file("log/logger-config.yaml", Default::default()).unwrap();
    let config = Config::get_or_set_config()?;

    let args = app_args();
    if config.first_run {
        show_first_run_prompt()?;
    }

    match args.subcommand() {
        Some(("create", sub_matches)) => println!(
            "'rsm create' was used, tablename is: {:?}, and due is {:?}",
            sub_matches.get_one::<String>("tablename").unwrap(),
            sub_matches.get_one::<bool>("due")
        ),
        Some(("delete", sub_matches)) => println!(
            "'rsm delete' was used, tablename is: {:?}",
            sub_matches.get_one::<String>("tablename").unwrap()
        ),
        Some(("list", sub_matches)) => println!(
            "'rsm list' was used, tablename is: {:?}, group is: {:?}, sort key is: {:?}",
            sub_matches.get_one::<String>("tablename"),
            sub_matches.get_one::<String>("group"),
            sub_matches.get_one::<String>("sort-by")
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}

fn show_first_run_prompt() -> Result<()> {
    todo!()
}
