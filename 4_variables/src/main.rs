// this is a constant on the global scope
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


fn main() {
    println!("==MUTABILITY==");
    mutability();

    println!("==SHADOWING==");
    shadowing();
}


fn mutability() {
    /*
    This will not work, as x is immutable:

    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println("The value of x is: {}", x);
    */

    // Now, this will work, as x is mutable:
    let mut x = 5;
    println!("The value of x is: {}", x); // -> 5
    x = 6;
    println!("The value of x is: {}", x); // -> 6
}

fn shadowing() {
    // We can also shadow a variable variable with a new one:

    // x is an immutable variable
    let x = 1; // x = 1

    // this x shadows the previous x, even if it's immutable
    let x = x + 1; // x = 1 + 1 = 2

    // this is a scope, it has its own declarations but shares the ones on the outer scope (the scope of the shadowing function)
    {
        // this x shadows the previous x of the outer scope, but only inside the inner scope
        let x = x * 2; // x = 2 * 2 = 4
        println!("The value of x (inner) is: {}", x); // -> 4
    }
    println!("The value of x (outer) is: {}", x); // -> 2

    // we can event change the type of a variable with shadowing!
    let spaces = "   ";
    let spaces = spaces.len();

    /*
    While this will not work:
    let mut spaces = "   ";
    spaces = spaces.len();
     */
}
