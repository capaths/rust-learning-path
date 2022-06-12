// enum simple definition
enum IpAddrKind {
    V4,
    V6,
}

// enum with variants
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// a more complex enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// implementation of methods for an enum
impl Message {
    fn call(&self) {
        println!("a message");
    }
}

fn main() {
    // enum instantiation
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // IpAddr instantiation
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // enum with variants
    let m = Message::Write(String::from("hello"));
    m.call();
}


// the Option enum is a type that represents a value that may or may not exist.
// Rust has no null!!!
fn option_enum() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;


    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; will cause an error, as y could be None
}
