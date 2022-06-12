use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Enter which number of the Fibonacci sequence you want to get: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n: u32 = input.trim().parse().expect("Please type a number!");

    if n > 186 {
        println!("The number you entered is too large to calculate with the loop algorithm.");
        return;
    }

    println!("Enter whether to use the recursive, or loop (default) algorithm (r/l): ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input = input.trim().to_lowercase();
    if input.starts_with("r") {
        println!("Using recursive algorithm");
        println!("The {}th fibonacci number is: {}", n, fibonacci(n));
    } else {
        println!("Using loop algorithm");
        println!("The {}th fibonacci number is: {}", n, fibonacci_loop(n));
    }
}

fn fibonacci(n: u32) -> u128 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn fibonacci_loop(n: u32) -> u128 {
    let mut previous_number: u128 = 0;
    let mut number: u128 = 1;
    for _ in 2..(n + 1) {
        let next_number = previous_number + number;
        previous_number = number;
        number = next_number;
    }
    number
}
