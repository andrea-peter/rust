use std::fs::File;
use std::io::{ErrorKind, Read};

/// Return text file content, empty string if it does not exist,
/// panic if any error occurs
fn read_text_file(file_path: &str) -> String {
    // We could use std::fs::read_to_string to read file content

    // This will return an error Result if the file does not exist
    let open_result = File::open(file_path);

    // We use an Option<> to indicate the file could be None
    let greeting_file: Option<File> = match open_result {
        Ok(file) => Some(file),
        // Further match error too see what kind of error it is
        Err(error) => match error.kind() {
            // If file not found the file will be None, otherwise panic
            ErrorKind::NotFound => None,
            _ => panic!("Could not read {file_path}: {error}"),
        },
    };

    let mut content = String::new();

    // If file is given (not None) read it, otherwise "read" empty string
    match greeting_file {
        Some(mut file) => match file.read_to_string(&mut content) {
            Ok(_) => content,
            Err(error) => panic!("Could not read file: {error}"),
        },
        None => {
            // Empty string if files does not exist
            String::from("")
        }
    }
}

fn main() {
    // Read file that exists
    let file_path = "hello.txt";
    println!("Reading file {file_path}");
    let file_content = read_text_file(file_path);
    println!("Got: {file_content}");

    // Read file that does not exists, we should get an empty string
    let file_path = "does_not_exists.txt";
    println!("Reading file {file_path}");
    let file_content = read_text_file(file_path);
    println!("Got: {file_content}");
}
