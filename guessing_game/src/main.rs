// import the std::io module to receive input from the user
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // `let` declares a new variable binding
    // `mut` indicates this binding may change (i.e. it's a mutable binding), by default variables are immutable
    // `String::new()` returns a new, empty `String` value.
    let mut guess = String::new();

    // stdin returns an instance of the standard input stream (std::io::Stdin) that represents the standard input of the current process
    io::stdin()
        // `read_line` reads a line from the standard input stream and storing it in the provided buffer
        .read_line(
            // the & means that it's passing a reference to the `String` value, and not a copy
            // by default the reference is immutable, but we can make it mutable by adding `mut`
            &mut guess
        )
        // ``read_line` returns a `Result` which has two variants: `Ok` and `Err`
        // we must expect the result, to throw an error when something goes wrong using the `expect` method
        .expect("Failed to read line");

    // println! can format a string using the `{}` placeholder, there could be multiple placeholders and multiple arguments
    println!("You guessed: {}", guess);
}
