pub fn main() {
    // creating an empty string
    let mut _s = String::new();

    // creating a string with a value
    let data = "initial contents";

    // creating a string from a string literal
    let _s = data.to_string();

    // the method also works on a literal directly:
    let _s = "initial contents".to_string();

    // accepts UTF-8!
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // update a string
    let mut s = String::from("foo");
    s.push_str("bar");

    // concatenate a string
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // push a char
    let mut s = String::from("lo");
    s.push('l');

    // concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // the + operator uses the add method

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // format
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    print!("{}", s);

    // the following code is confused because of the UTF-8 encoding, so RUST won't let you do it
    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    // slicing
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    // iterating chars
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // iterating bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
