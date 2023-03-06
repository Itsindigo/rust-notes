use std::fs::File;
use std::io::{self, Read};

fn main() {
    let read_username_result = read_username_from_file();

    match read_username_result {
        Ok(u) => println!("Username: {}", u.trim()),
        Err(e) => panic!("Error reading username: {:?}", e)
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file, // returns `file` from the match expression.
        Err(e) => return Err(e), // returns from the top level function with a Result::Error<io::Error>
    };

    let mut username = String::new();

    // Implicitly returns either the Ok or Err arm.
    // 
    // NOTE: with read_to_string, `file` is source and `username` arg is target.
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}
