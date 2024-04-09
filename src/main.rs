// https://docs.rs/clap/latest/clap/_tutorial/chapter_0/index.html

use std::io::Write;
use std::io;
use std::{collections::HashMap, path::PathBuf};

use clap::{command, value_parser, Arg, ArgAction, ArgGroup, Command};
use parsers::LineRange;
use utils::config_helper::{Config, Token};

use crate::api::{ErrorResponse, SuccessfulResponse};
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
        .subcommand(Command::new("logout").about("Logout from the account"))
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

    let args = app_args();
    

    let mut api = if !args.subcommand_matches("new-key").is_some() {
        if config.first_run {
            let api = Api::new_without_token();
            show_first_run_prompt(&api, &mut config)?;
            config.first_run = false;
            config.update_config()?;
        } 
        Api::new()?
    } else {
        Api::new_without_token()
    };


    match args.subcommand() {
        Some(("new-key", _)) => {
            println!("DEBUG:'rsm new-key' was used");

            println!("Please input your credentials: \n");
            print!("username: ");
            io::stdout().flush().map_err(|_| Error::RsmFailed)?;

            let mut username = String::new();
            io::stdin().read_line(&mut username).map_err(|_| Error::RsmFailed)?;
            
            let password = rpassword::prompt_password("password: ").map_err(|_| Error::RsmFailed)?;

            let handle = terminal_spinners::SpinnerBuilder::new().spinner(&terminal_spinners::DOTS).text("Making a new key...").start();
            let res = api.post_lostkey(&username, &password)?;
            handle.done();
            log::info!("Successfully sent GET lostkey request and received response");

            let res_type = &res.as_any();
            if res_type.is::<ErrorResponse>() {
                res.print();
                return Err(Error::FailedToUpdateKey);
            } else if res_type.is::<SuccessfulResponse>() {
                res.print();
                println!("\x1b[34mNow login again\x1b[0m\n");
                config.first_run = true;
                config.update_config()?;

                let (key, token) = login(&api).map_err(|e| {log::error!("{e:?}"); e})?;
                config.key = Some(key.0.replace("\n", ""));
                let token: String = token.into();
                config.token = Some(token.replace("\n", ""));
                config.first_run = false;
                config.update_config()?;

                log::info!("successful key change process");
            }
        },
        Some(("logout", _)) => {
            println!("DEBUG:'rsm logout' was used");

            print!("Do you really want to log out(yes, [no]): ");
            std::io::stdout().flush().map_err(|_| Error::RsmFailed)?;
            let choice = get_user_choice().map_err(|_| Error::RsmFailed)?;

            let logout: bool = match choice {
                Choice::Yes => true,
                Choice::No => false,
            };

            match api.post_logout(logout) {
                Ok(res) => {
                    log::info!("Successfully sent GET logout request and received response");
                    if logout {
                        // reset config
                        config.token = None;
                        config.first_run = true;
                        config.key = None;
                    }
                    res.print();
                },
                Err(err) => {
                    log::error!("Error occurred while fetching tasks: {:?}", err);
                    return Err(err);
                }
            }
            config.update_config()?;
            api.update_token()?;
        },
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
                    log::info!("Successfully sent GET list request and received response");
                    res.print();
                },
                Err(err) => {
                    log::error!("Error occurred while fetching tasks: {:?}", err);
                    return Err(err);
                }
            }
        },
        // TODO:
        Some(("create", sub_matches)) => println!(
            "'rsm create' was used, tablename is: {:?}, and due is {:?}",
            sub_matches.get_one::<String>("tablename").unwrap(),
            sub_matches.get_one::<bool>("due")
        ),
        // TODO:
        Some(("drop", sub_matches)) => println!(
            "'rsm drop' was used, tablename is: {:?}",
            sub_matches.get_one::<String>("tablename").unwrap()
        ),
        // TODO:
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
        // TODO:
        Some(("remove", sub_matches)) => println!(
            "'rsm remove' was used, tablename is: {:?}, id is {:?}",
            sub_matches.get_one::<String>("tablename").unwrap(),
            sub_matches.get_one::<u8>("id").unwrap(),
        ),
        // TODO:
        Some(("update", sub_matches)) => println!(
            "'rsm update' was used, tablename is: {:?}, id is {:?}",
            sub_matches.get_one::<String>("tablename").unwrap(),
            sub_matches.get_one::<u8>("id").unwrap()
        ),
        // TODO:
        Some(("clear", sub_matches)) => println!(
            "'rsm clear' was used, tablename is: {:?}",
            sub_matches.get_one::<String>("tablename").unwrap()
        ),
        _ => unreachable!("If you are reading this something really bad happened"),
    }

    Ok(())
}

fn show_first_run_prompt(api: &Api, config: &mut Config) -> Result<()> {
    println!("\x1b[34mWelcome to RsMember!\x1b[0m\n");

    print!("do you already have a key([yes]/no): ");
    std::io::stdout().flush().map_err(|_| Error::RsmFailed)?;
    let choice = get_user_choice().map_err(|_| Error::RsmFailed)?;

    match choice {
        // send login req
        Choice::Yes => {
            let (key, token) = login(api).map_err(|e| {log::error!("{e:?}"); e})?;
            config.key = Some(key.0.replace("\n", ""));
            let token: String = token.into();
            config.token = Some(token.replace("\n", ""));

            log::info!("successful login");
            Ok(())
        },
        // send signup req
        Choice::No => {
            signup(api).map_err(|e| {log::error!("{e:?}"); e})?;
            println!("Log in:");

            let (key, token) = login(api).map_err(|e| {log::error!("{e:?}"); e})?;
            config.key = Some(key.0.replace("\n", ""));
            let token: String = token.into();
            config.token = Some(token.replace("\n", ""));

            log::info!("successful signup and login");
            Ok(())
        },
    }
}

struct Key(String);

impl From<String> for Key {
    fn from(value: String) -> Key {
        Key(value)
    }
}

fn login(api: &Api) -> Result<(Key, Token)> {
    println!("Please input your key");

    let mut key = String::new();
    io::stdin().read_line(&mut key).map_err(|_| Error::RsmFailed)?;

    let handle = terminal_spinners::SpinnerBuilder::new().spinner(&terminal_spinners::DOTS).text("Signing up...").start();
    let res = api.post_login(&key)?;
    handle.done();

    let res_type = &res.0.as_any();
    if res_type.is::<ErrorResponse>() {
        res.0.print();
        return Err(Error::LoginFail);
    } else if res_type.is::<SuccessfulResponse>() {
        res.0.print();
        println!("\x1b[34mWelcome to this machine!\x1b[0m\n");
    }
    Ok((key.into(), res.1.into()))
}

fn signup(api: &Api) -> Result<()> {
    println!("Create Account:");
    print!("username: ");
    io::stdout().flush().map_err(|_| Error::RsmFailed)?;

    let mut username = String::new();
    io::stdin().read_line(&mut username).map_err(|_| Error::RsmFailed)?;
    
    let password = rpassword::prompt_password("password: ").map_err(|_| Error::RsmFailed)?;

    let handle = terminal_spinners::SpinnerBuilder::new().spinner(&terminal_spinners::DOTS).text("Signing up...").start();
    let res = api.post_signup(&username, &password)?;
    handle.done();

    let res_type = &res.as_any();
    if res_type.is::<ErrorResponse>() {
        res.print();
        return Err(Error::FirstRunFailed);
    } else if res_type.is::<SuccessfulResponse>() {
        println!("Account creation successful, you can now log in!");
        res.print();
    }
    Ok(())
}
