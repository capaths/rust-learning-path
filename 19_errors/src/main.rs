// when panic! is called, the program will exit and unwind the stack unless Cargo.toml has the following:
// [profile.release]
// panic = 'abort'
//
// Which just stops the program skipping the unwinding of the stack.


// Recoverable errors are handled using the Result struct which looks like this:
//
// enum Result<T, E> {
//    Ok(T),
//    Err(E),
// }
//

use std::{fs::File, io, io::{ErrorKind, Read}};

fn main() {
    recoverable_error();
}


fn recoverable_error() {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            _ => {
                panic!("There was a problem opening the file: {:?}", error);
            }
        }
    };
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap_or_else(|error| {
        panic!("There was a problem opening the file: {:?}", error);
    });
    println!("{}", content);

    let _f = File::open("hello.txt").unwrap(); // will panic if file is not found

    let _f = File::open("hello.txt").expect("File not found"); // will panic with the given message if file is not found
}


// propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


// propagating errors with shortcut
fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// propagating errors with shortcut
fn read_username_from_file_shortcutter() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?; // the ? will lose the original return value if errored

    Ok(s)
}
