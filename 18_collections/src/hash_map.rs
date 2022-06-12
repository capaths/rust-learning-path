use std::collections::HashMap;


pub fn main() {
    // by default, a hash map uses strings as keys and integers as values
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // using initial values from vectors vector
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams
        .into_iter()
        .zip(
            initial_scores.into_iter()
        )
        .collect(); // collect transforms the given iterators into a collection

    // only inserting a value if the key is not already present
    scores.entry(String::from("Blue")).or_insert(50);

    // count the number of words in a string
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // increment to the derreferenced value
    }

    println!("{:?}", map);
}
