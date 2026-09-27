//! SQLite example
//!
//! A simple user database management tool

use rusqlite::{Connection, OpenFlags, Result};
use std::fs;

// Database table
#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub address: Option<String>,
}

/// Open the given DB file in read only mode and returns the connection
pub fn open_db_ro(db_path: &String) -> Result<Connection> {
    Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)
}

/// Opens the given DB file in read-write mode and returns the connection
pub fn open_db_rw(db_path: &String) -> Result<Connection> {
    Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_WRITE)
}

/// Creates a database in the given DB file and returns the connection
///
/// # Panics
///
/// If the file already exists and `overwrite` is false this function will panic
pub fn create_db(db_file: &String, overwrite: bool) -> Result<Connection> {
    if fs::exists(db_file).unwrap() {
        if overwrite {
            println!("Removing existing DB '{db_file}'");
            fs::remove_file(db_file).expect("Could not remove existing DB");
        } else {
            panic!("File already exists");
        }
    }
    Connection::open_with_flags(
        db_file,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )
}

/// Creates the `user` table
pub fn create_user_table(conn: &Connection) -> Result<usize> {
    conn.execute(
        "CREATE TABLE user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            address TEXT NULL
        )",
        (),
    )
}

/// Returns a vector of all the users in the DB
pub fn get_users(conn: &Connection) -> Result<Vec<User>> {
    let mut stmt = conn.prepare("SELECT id, name, address FROM user")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            address: row.get(2)?,
        })
    })?;
    let mut users: Vec<User> = Vec::new();
    for user in user_iter {
        users.push(user.expect("Error creating User from row"));
    }
    return Ok(users);
}

pub fn add_user(conn: &Connection, name: &String, address: &Option<String>) -> Result<usize> {
    conn.execute(
        "INSERT INTO user (name, address) VALUES (?1, ?2)",
        (name, address),
    )
}

pub fn remove_user(conn: &Connection, id: u32) -> Result<usize> {
    conn.execute("DELETE FROM user where id=?1", (id,))
}
