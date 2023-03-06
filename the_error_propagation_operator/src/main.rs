use std::fs::File;
use std::io::{self, Read}

fn main() {
    read_username_from_file();
}


/** 
 * The ? can be used to propagate Err(e) variants from a Result,
 * the `?` mark passes the Result through a `From` trait's `from`
 * function, allowing the result to be unpacked into diff result
 * types if necessary.
*/
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
