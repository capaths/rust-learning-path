// import the std::io module to receive input from the user
use std::io;

// import the std::cmp module to compare numbers
use std::cmp::Ordering;

// import the rand module to generate random numbers
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::
        // gets a random generator
        thread_rng()
    // generate a random number between 1 and 100
    // start..end is a range inclusive on start and exclusive on end
    .gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    // we allow multiple guesses by looping
    loop {
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
                &mut guess,
            )
            // ``read_line` returns a `Result` which has two variants: `Ok` and `Err`
            // we must expect the result, to throw an error when something goes wrong using the `expect` method
            .expect("Failed to read line");

        // convert the guess to a number
        // `parse` converts a string to a number
        // `parse` returns a `Result`
        // this guess, shadows the previous guess variable

        // match is similar to a switch statement, but it can also be used to return a value
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue
        };

        // println! can format a string using the `{}` placeholder, there could be multiple placeholders and multiple arguments
        println!("You guessed: {}", guess);

        // match is like a switch statement in other languages, where each case is called an arm
        // guess.cmp is a method that compares the guess to the given number reference
        match guess.cmp(&secret_number) {
            // `Ordering` is an enum with three variants: `Less`, `Equal` and `Greater`
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
