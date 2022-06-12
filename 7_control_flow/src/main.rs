fn main() {
    println!("Hello, world!");
    if_else_exp();
    if_let();
    count();
    loop_return();
    while_conditional();
    looping_collection();
}

fn if_else_exp() {
    let number = 3;

    // classic if-else, does not need parentheses around the condition
    if number < 5 {
        println!("number is lower than 5");
    } else if number > 5 {
        println!("number is higher than 5");
    } else {
        println!("number is 5");
    }
}

fn if_let() {
    let condition = true;
    // When using an if-else to set a let expression, the result must always be of the same type.
    let _number = if condition { 5 } else { 6 };
}

fn count() {
    let mut count = 0;
    // the 'counting_loop: is a label, and is useful to break an specific loop
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                // break will break out of the inner loop, but not the outer loop
                break;
            }
            if count == 2 {
                // break the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}

fn loop_return() {
    let mut counter = 0;

    // we can store the return value of a loop in a variable
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_conditional() {
    let mut number = 3;

    // while is a loop that runs while a condition is true
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn looping_collection() {
    let a = [10, 20, 30, 40, 50];

    // for-in is a loop that iterates over a collection
    for element in a {
        println!("the value is: {}", element);
    }

    // you could use a while loop too
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // or use a for-in loop to iterate over a range
    // here, .rev() reverses the range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
