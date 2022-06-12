fn main() {
    scope();
    string_type();
    interaction();
    deep_copy();
    copy();
    functions();
    functions_return();
    transferring_ownership();
}


fn scope() {
    // s is not valid here
    {
        // s is valid from the next line until the end of the scope
        let _s = "hello";
    }
    // s is not valid anymore
}

fn string_type() {
    // String are allocated on the heap because of their complex structure
    let mut s = String::from("hello"); // here, the memory is allocated from the literal "hello"

    // for example, if mutable, it can mutate!
    s.push_str(", world!");

    println!("{}", s);

    // what rust does is that when we leave the scope, the string is dropped, and the memory is freed
}

fn interaction() {
    // in this code, the value from x is copied to y, as integers are of fixed size and thus, stored in the stack.
    let _x = 5;
    let _y = _x;

    // meanwhile, with strings, the reference is the same, a copy is made...
    // but of the pointer of the start of the string and some other metadata, not the content itself,
    // as it's stored in the heap
    let _s1 = String::from("hello");
    let _s2 = _s1;

    // so, if we could change the value of s1, it will affect s2
    // but rust doesn't allow this, after s2 is set, s1 is no longer valid
}

fn deep_copy() {
    // to create a copy of some data in the heap, a common way is to use the clone method
    let _s1 = String::from("hello");
    let _s2 = _s1.clone();

    println!("s1 = {}, s2 = {}", _s1, _s2);
    // and now we can use both s1 and s2!
}

fn copy() {
    // types stored in the stack can have a Copy trait, which means that they can be copied
    // if the trait is implemented, a variable is still valid even after assigned to another variable
    // but consider, if the Drop trait is implemented for a type, that type cannot have the Copy trait

    // these types have the Copy trait:

    let _i1 = 5;
    let _i2 = _i1;

    let _b1 = true;
    let _b2 = false;

    let _f1 = 3.14;
    let _f2 = _f1;

    let _c1 = 'c';
    let _c2 = _c1;

    let _t1 = (5, 6); // tuples only have the Copy trait if all their elements have it too
    let _t2 = _t1;
}


fn functions() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;
    makes_copy(x);

    // we could not use s here, but we could use x

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn functions_return() {
    let _s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let _s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(_s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}



fn transferring_ownership() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length) // returning a tuple, returns the ownership, but there must be another way... references?
}
