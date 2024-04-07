// https://docs.rs/clap/latest/clap/_tutorial/chapter_0/index.html

use std::{collections::HashMap, path::PathBuf};

use clap::{command, value_parser, Arg, ArgAction, ArgGroup, Command};
use parsers::LineRange;
use utils::config_helper::Config;

use crate::utils::{get_user_choice, Choice};
use crate::{api::Api, error::Error};
use crate::error::Result;

pub mod api;
pub mod error;
pub mod parsers;
pub mod utils;

fn app_args() -> clap::ArgMatches {
    command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("new-key").about("Resets the account key")) //TODO: confirmation
        .subcommand(Command::new("logout").about("Logout from the account")) // TODO confirmation
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
            Command::new("drop").about("Deletes a table").arg(
                Arg::new("tablename")
                    .required(true)
                    .help("Name of the table to remove"),
            ),
        )
        .subcommand(
            Command::new("add")
                .about("Adds a task into a table")
                .arg(
                    Arg::new("tablename")
                        .required(true)
                        .help("Name of the table where to add the task"),
                )
                .group(
                    ArgGroup::new("source")
                        .required(true)
                        .args(&["task", "file"]),
                )
                .arg(
                    Arg::new("task")
                        .long("task")
                        .short('t')
                        .conflicts_with("file")
                        .help("The task to add as text")
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    Arg::new("file")
                        .long("file")
                        .short('f')
                        .conflicts_with("task")
                        .help("File to add task from")
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    Arg::new("line")
                        .long("line")
                        .short('l')
                        .requires("file")
                        .help("Add task from a specific line")
                        .value_parser(value_parser!(u16)), // non negative number
                )
                .arg(
                    Arg::new("range")
                        .long("range")
                        .short('r')
                        .value_name("START..END")
                        .requires("file")
                        .help("Add task from a range")
                        .value_parser(value_parser!(LineRange)),
                )
                .arg(
                    Arg::new("due")
                        .long("due")
                        .short('d')
                        .help("The due of the task")
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    Arg::new("group")
                        .long("group")
                        .short('g')
                        .help("The group of the task"),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Removes a task from a table")
                .arg(
                    Arg::new("tablename")
                        .required(true)
                        .help("Name of the table where to remove the task"),
                )
                .arg(
                    Arg::new("desc")
                        .required(true)
                        .help("The description of the task to remove")
                        .value_parser(value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("Updates a task from a table")
                .arg(
                    Arg::new("tablename")
                        .required(true)
                        .help("Name of the table where to update the task"),
                )
                .arg(
                    Arg::new("desc")
                        .required(true)
                        .help("The description of the task to remove")
                        .value_parser(value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("clear")
                .about("Clears completely a table")
                .arg(
                    Arg::new("tablename")
                        .required(true)
                        .help("Name of the table where to clear"),
                ),
        )
        .get_matches()
}

fn main() -> Result<()> {
    // init logger
    log4rs::init_file("log/logger-config.yaml", Default::default()).unwrap();

    //init config and if it is the first time running show the default prompt
    let mut config = Config::get_config()?;

    let api = Api::new()?;

    let args = app_args();
    if config.first_run {
        show_first_run_prompt()?;
        config.first_run = false;
        config.update_config()?;
    }

    match args.subcommand() {
        Some(("new-key", _)) => println!("'rsm new-key' was used"),
        Some(("logout", _)) => println!("'rsm logout' was used"),
        Some(("list", sub_matches)) => { 
            
            let tablename = sub_matches.get_one::<String>("tablename").map(|s| s.as_str());
            let group = sub_matches.get_one::<String>("group").map(|s| s.as_str());
            let sort_key = sub_matches.get_one::<String>("sort-by").map(|s| s.as_str());

            println!("DEBUG: 'rsm list' was used, tablename is: {:?}, group is: {:?}, sort key is: {:?}", tablename, group, sort_key);

            let mut opts_map: HashMap<&str, &str> = HashMap::new();
            if let Some(group_value) = group {
                opts_map.insert("group", group_value);
            }
            if let Some(sort_by_value) = sort_key {
                opts_map.insert("sort_by", sort_by_value);
            }

            match api.get_tasks(tablename, opts_map) {
                Ok(res) => {
                    log::info!("Successfully sent GET request and received response");
                    res.print();
                },
                Err(err) => {
                    log::error!("Error occurred while fetching tasks: {:?}", err);
                    return Err(err);
                }
            }
        },
        Some(("create", sub_matches)) => println!(
            "'rsm create' was used, tablename is: {:?}, and due is {:?}",
            sub_matches.get_one::<String>("tablename").unwrap(),
            sub_matches.get_one::<bool>("due")
        ),
        Some(("drop", sub_matches)) => println!(
            "'rsm drop' was used, tablename is: {:?}",
            sub_matches.get_one::<String>("tablename").unwrap()
        ),
        Some(("add", sub_matches)) => println!(
            "'rsm add' was used, tablename is: {:?}, task is {:?}, file is {:?}, line is {:?}, range is {:?}, due is {:?}, group is {:?}",
            sub_matches.get_one::<String>("tablename"),
            sub_matches.get_one::<String>("task"),
            sub_matches.get_one::<PathBuf>("file"),
            sub_matches.get_one::<u16>("line"),
            sub_matches.get_one::<LineRange>("range"),
            sub_matches.get_one::<String>("due"),
            sub_matches.get_one::<String>("group"),
        ),
        Some(("remove", sub_matches)) => println!(
            "'rsm remove' was used, tablename is: {:?}, id is {:?}",
            sub_matches.get_one::<String>("tablename").unwrap(),
            sub_matches.get_one::<u8>("id").unwrap(),
        ),
        Some(("update", sub_matches)) => println!(
            "'rsm update' was used, tablename is: {:?}, id is {:?}",
            sub_matches.get_one::<String>("tablename").unwrap(),
            sub_matches.get_one::<u8>("id").unwrap()
        ),
        Some(("clear", sub_matches)) => println!(
            "'rsm clear' was used, tablename is: {:?}",
            sub_matches.get_one::<String>("tablename").unwrap()
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}

/*
Welcome to RsMember!

do you already have a key([yes]/no)
no
Account creation:
username: tizio123
password: *****

Your key is: 982y3hfhaajdf9011ahdf
Account creation successful! 

Use it in other devices to sync to your account

Thank you!
*/

fn show_first_run_prompt() -> Result<()> {
    use std::io;
    use std::io::Write;

    println!("\x1b[34mWelcome to RsMember!\x1b[0m\n");

    let choice = get_user_choice().map_err(|_| Error::RsmFailed)?;

    match choice {
        Choice::Yes => todo!("IMPLEMENT YES PATH"),
        Choice::No => {

            println!("Create Account:");
            print!("username: ");
            io::stdout().flush().map_err(|_| Error::RsmFailed)?;

            let mut username = String::new();
            io::stdin().read_line(&mut username).map_err(|_| Error::RsmFailed)?;
            
            let password = rpassword::prompt_password("password: ").map_err(|_| Error::RsmFailed)?;
            // TODO: send this to login
            println!("DEBUG:{username}");
            println!("DEBUG:{password}");
        },
    }

    println!("Thank you!");
    unimplemented!("create the account calling signup on the api");

    Ok(())
}
