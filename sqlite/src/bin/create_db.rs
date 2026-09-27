use std::env::var;
use std::fs;

use clap::Parser;
use rusqlite::{Connection, Result};

// Command line parameters
#[derive(Parser)]
struct Args {
    out_file: String,
    #[arg(short = 'f', help = "Force file creation")]
    force: bool,
}

// Database table
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    address: Option<String>,
}

fn main() -> Result<()> {
    // Get command line args
    let args = Args::parse();

    // Check file exists
    if fs::exists(&args.out_file).unwrap() {
        if args.force {
            println!("File '{}' already exists, removing", args.out_file);
            fs::remove_file(&args.out_file).unwrap();
        } else {
            panic!(
                "OUTFILE {} exists, pass the `-f` option to overwrite",
                args.out_file
            );
        }
    }

    println!("Writing DB to {}", args.out_file);

    let conn = Connection::open(&args.out_file)?;

    println!("Creating table 'user'");
    conn.execute(
        "CREATE TABLE user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            address TEXT NULL
        )",
        (),
    )?;

    let user_name = var("USER")
        .expect("Env variable USER not set")
        .replace('"', "");

    println!("Adding default user '{user_name}'",);
    conn.execute(
        "INSERT INTO user (name, address) VALUES (?1, NULL)",
        (&user_name,),
    )?;

    println!("Current DB content");
    println!("Users: ");
    let mut stmt = conn.prepare("SELECT id, name, address FROM user")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            address: row.get(2)?,
        })
    })?;
    for user in user_iter {
        println!(" - {:?}", user.unwrap());
    }

    Ok(())
}
