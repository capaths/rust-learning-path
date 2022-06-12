use std::io;

fn main() {

    let mut input = String::new();

    println!("Enter a temperature in Farenheit or Celsius: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: f32 = input.trim().parse().expect("Please type a number!");

    println!("{}F is {}C", input, to_celsius(input));
    println!("{}C is {}F", input, to_farenheit(input));
}

fn to_celsius(farenheit: f32) -> f32 {
    (farenheit - 32.0) * (5.0 / 9.0)
}

fn to_farenheit(celsius: f32) -> f32 {
    (celsius * (9.0 / 5.0)) + 32.0
}
