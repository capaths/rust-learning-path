fn main() {
    let mut s1 = String::from("hello");  // s1 comes into scope

    let len = calculate_length(&s1); // we pass the address of s1 (using &)
                                     // instead of the String itself

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);

    let len = calculate_length(&s1); // we pass the address of s1 (using &)
                                     // instead of the String itself

    println!("The length of '{}' is {}.", s1, len);

    multiple_references();
    
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // but, we can't modify s, because it's passed as a reference
    // s.push_str(", world"); // cannot borrow as mutable, as it is behind a reference
}

// we can mutate a string with &mut String, but...
fn change(some_string: &mut String) {
    some_string.push_str(", world");
    // there's one big restriction:
    // only one mutable reference can exist for a particular piece of data
    // nor have immutable references if there's a mutable reference of the same value

    // but! that means no dangling pointers!
}


fn multiple_references() {
    // but we could have multiple references to the same data
    // just not simultaneously
    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    }
    let _r2 = &mut s;
}