fn main() {
    // we can define types with a colon and the type name after the variable name
    let _guess: u32 = "42".parse().expect("Not a number!");

    // or, if using the result of a function, by using ::<TYPE> syntax after a function with a type signature
    let _guess = "42".parse::<u32>().expect("Not a number!");

    tuple();
    array();
}


fn tuple() {
    let tup: (i32, f64, i32) = (500, 6.4, 1);

    // it's possible to destructure a tuple
    let (_x, _y, _z) = tup;

    println!("The value of y is: {}", _y);

    // it's also possible to access a value by index
    println!("The value of the third element is: {}", tup.2);
}

fn array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // it's possible to access an element by index
    println!("The value of the third element is: {}", a[2]);
}
