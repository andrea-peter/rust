//! CLI for managing users database
use clap::{Parser, Subcommand};
use sqlite_example::{add_user, get_users, open_db_ro, open_db_rw, remove_user};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(long, value_name = "FILE")]
    db: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        address: Option<String>,
    },
    Delete {
        id: u32,
    },
    List {},
}

fn main() {
    let cli = Cli::parse();
    let db_path: &String = &cli.db.into_string().unwrap();

    match &cli.command {
        Commands::Add { name, address } => {
            let conn = open_db_rw(db_path).unwrap();
            add_user(&conn, name, address).unwrap();
        }
        Commands::Delete { id } => {
            let conn = open_db_rw(db_path).unwrap();
            let rows_removed = remove_user(&conn, *id).unwrap();
            if rows_removed == 0 {
                println!("No such user");
            }
        }
        Commands::List {} => {
            let conn = open_db_ro(db_path).unwrap();
            println!("Users:");
            for user in get_users(&conn).unwrap() {
                println!(
                    " - {}. {} - {}",
                    user.id,
                    user.name,
                    user.address.unwrap_or(String::from("NULL"))
                );
            }
        }
    }
}
