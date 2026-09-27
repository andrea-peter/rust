use std::env::var;

use clap::Parser;
use rusqlite::Result;

use sqlite_example::{add_user, create_db, create_user_table, get_users};

// Command line parameters
#[derive(Parser)]
struct Args {
    out_file: String,
    #[arg(short = 'f', help = "Force file creation")]
    force: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Creating DB at {}", args.out_file);
    let conn = create_db(&args.out_file, args.force).expect("Could not create database");

    println!("Creating table 'user'");
    create_user_table(&conn).expect("Could not create user table");

    let user_name = var("USER")
        .expect("Env variable USER not set")
        .replace('"', "");

    println!("Adding default user '{user_name}'",);
    add_user(&conn, &user_name, &None).expect("Could not add default user");

    println!("Current DB content");
    println!("Users: ");
    for user in get_users(&conn)? {
        println!(" - {:?}", user);
    }

    Ok(())
}
