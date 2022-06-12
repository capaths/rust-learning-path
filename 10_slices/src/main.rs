fn main() {
    let s = String::from("hello world!");  // s comes into scope

    let _word_index = first_word(&s);          // word comes into scope
    // but! if s is cleared, wordIndex will be obsolete and error prone

    // so we use string slices
    let word = first_word_slice(&s);

    // s.clear(); will cause error on compile time

    println!("The first word is: {}", word);

    // slices can also be used on arrays
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    println!("{:?}", slice);
}

// s is a reference as we don't want ownership
fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();                    // convert to an array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iterate over the bytes
        if item == b' ' { // check if the byte is a space
            return i; // return the index of the space
        }
    }

    s.len() // if we didn't find a space, return the length of the string
}

// s is a reference as we don't want ownership
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();                    // convert to an array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iterate over the bytes
        if item == b' ' { // check if the byte is a space
            return &s[0..i]; // return the index of the space
        }
    }

    &s[..] // if we didn't find a space, return the whole string
}
