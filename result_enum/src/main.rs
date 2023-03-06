use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // returns a `Result<std::fs::File, std::io::Error>`
    let greeting_file_result = File::open("hello.txt");

    // if the line above succeeds, then greeting_file_result will
    // be an instance of `Ok(std::fs::File)`, but if it fails
    // then it will be an instance of `Err(std::io::Error)`.

    // We need logic which can handle these two scenarios.

    // Note: That a `match` statement returns, and so can be
    // used as an assignment expression.
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html#variants
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };


    // THE ABOVE MATCH STATEMENT COULD ALSO BE EXPRESSED USING CLOSURES:
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
