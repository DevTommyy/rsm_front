use api::Api;
use clap::{error::Result, Args, Parser, Subcommand};
use utils::{parse_due, prompt_logout, Due};

mod api;
mod formatter;
mod utils;

#[derive(Parser, Debug)]
#[command(
    name = "rsmember",
    about = "Basically a \"todo\" app for the cli",
    subcommand_required = true,
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Login into your account
    Login,
    /// Create a new account
    Signup,
    /// Logout from the account
    Logout,
    /// Creates a new table
    Create(CreateArgs),
    /// Deletes a table
    Drop(DropArgs),
    /// List table contents or tables with their specs
    List(ListArgs),
    /// Adds a task into a table
    Add(AddArgs),
    /// Removes a task from a table
    Remove(RemoveArgs),
    /// Updates a task from a table
    Update(UpdateArgs),
    /// Clears completely a table
    Clear(ClearArgs),
}

// create table
#[derive(Args, Debug)]
struct LoginArgs {
    #[arg(long = "username", short = 'u')]
    username: String,
    #[arg(long = "password", short = 'u')]
    pwd: String,
}

// create table
#[derive(Args, Debug)]
struct CreateArgs {
    tablename: String,
    #[arg(long = "due", short = 'd', requires = "tablename", action = clap::ArgAction::SetTrue)]
    due: bool,
    #[arg(long = "group", short = 'g', requires = "tablename", action = clap::ArgAction::SetTrue)]
    group: bool,
}

// drop table
#[derive(Args, Debug)]
struct DropArgs {
    tablename: String,
}

// list table items or all tables with specs if no tablename provided
#[derive(Args, Debug)]
struct ListArgs {
    tablename: Option<String>,
    #[arg(short = 'g', long = "group", requires = "tablename")]
    group: Option<String>,
    // can be either due or id, checked in the backend
    #[arg(short = 's', long = "sort-by", requires = "tablename")]
    sort_by: Option<String>,
}

// add a task to a table
#[derive(Args, Debug)]
struct AddArgs {
    tablename: String,
    #[arg(short = 't', long = "task", requires = "tablename")]
    task: String,
    #[arg(short = 'd', long = "due", requires = "tablename", value_parser = parse_due)]
    due: Option<Due>,
    #[arg(short = 'g', long = "group", requires = "tablename")]
    group: Option<String>,
}

// remove a task from a table
#[derive(Args, Debug)]
struct RemoveArgs {
    tablename: String,
    #[arg(requires = "tablename")]
    id: String,
}

// update a task of a table
#[derive(Args, Debug)]
struct UpdateArgs {
    tablename: String,
    #[arg(requires = "tablename")]
    id: String,
    #[arg(short = 't', long = "task", requires = "id")]
    task: String,
    #[arg(short = 'd', long = "due", requires = "tablename", value_parser = parse_due)]
    due: Option<Due>,
    #[arg(short = 'g', long = "group", requires = "id")]
    group: Option<String>,
}

// clean a table
#[derive(Args, Debug)]
struct ClearArgs {
    tablename: String,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let api = Api::from_token_file();

    api.has_connection()?
        .then_some(())
        .ok_or_else(|| "No internet connection found, check your Wi-Fi connection".to_string())?;

    // if login or signup match beforehand
    match cli.command {
        Commands::Login => {
            let (usr, pwd) =
                utils::prompt_credentials().map_err(|e| format!("Internal error: {e}"))?;

            let res = api.login(usr, pwd)?;
            // res is there, it wont only if there'll be breaking changes on the api
            println!();
            println!(
                "Your token is: '{}', put it in your '.token' file",
                res.get("res").and_then(|v| v.as_str()).unwrap()
            );
            println!("successfully logged in");

            return Ok(());
        }
        Commands::Signup => {
            let (usr, pwd) =
                utils::prompt_credentials().map_err(|e| format!("Internal error: {e}"))?;

            let res = api.register_user(usr, pwd)?;
            println!();
            println!("{}", res.get("res").and_then(|v| v.as_str()).unwrap());
            println!("Now you can login");

            return Ok(());
        }
        _ => {}
    }

    // Ensure the user has a valid token before proceeding with other commands
    if !api.has_token() {
        return Err(
            "No token found, you must login or sign up and put your token in the '.token' file"
                .to_string(),
        );
    }

    // Now process remaining commands
    match cli.command {
        Commands::Logout => {
            let logout = prompt_logout().map_err(|e| format!("Internal error: {e}"))?;
            let res = api.logout(logout)?;

            //clear the .token file, no need to change the state of the api cause the program will
            //end right after this
            if let Err(e) = std::fs::File::create(".token") {
                panic!("couldnt clear token file: {e}")
            }

            println!("{res}");
            unimplemented!("format the res")
        }
        Commands::Create(CreateArgs {
            tablename,
            due,
            group,
        }) => {
            let res = api.create_table(&tablename, due, group)?;
            println!("{res}");
            unimplemented!("format the res")
        }
        Commands::Drop(DropArgs { tablename }) => {
            let res = api.drop_table(&tablename)?;
            println!("{res}");
            unimplemented!("format the res")
        }
        Commands::List(list_args) => {
            if let Some(tablename) = list_args.tablename {
                // list 'tablename' contents
                let res = api.list_table_contents(
                    &tablename,
                    list_args.group.as_deref(),
                    list_args.sort_by.as_deref(),
                )?;
                println!("{res}");
                unimplemented!("format the res")
            } else {
                // list table specs
                let res = api.list_tables_specs()?;
                println!("{res}");
                unimplemented!("format the res")
            };
        }
        Commands::Add(AddArgs {
            tablename,
            task,
            due,
            group,
        }) => {
            let res = api.add_task(&tablename, &task, due, group.as_deref())?;
            println!("{res}");
            unimplemented!("format the res")
        }
        Commands::Remove(RemoveArgs { tablename, id }) => {
            let res = api.remove_task(&tablename, &id)?;
            println!("{res}");
            unimplemented!("format the res")
        }
        Commands::Update(UpdateArgs {
            tablename,
            id,
            task,
            due,
            group,
        }) => {
            let res = api.update_task(&tablename, &id, &task, due, group.as_deref())?;
            println!("{res}");
            unimplemented!("format the res")
        }
        Commands::Clear(ClearArgs { tablename }) => {
            let res = api.clear_table(&tablename)?;
            println!("{res}");
            unimplemented!("format the res")
        }
        _ => unreachable!(), // This handles exhaustive checking without runtime cost
    }
}
