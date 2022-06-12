fn main() {
    println!("Hello, world!");
    another_function(42);
    print_labeled_measurement(42, 'm');
    expressions();
}

fn expressions() {
    let _x = 5 + five();
    let y = {
        let _x = 3;
        plus_one(_x)
    };
    println!("The value of y is: {}", y); // 4
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
