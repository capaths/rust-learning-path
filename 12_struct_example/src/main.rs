// the following line, allows the struct to be displayed using {:?}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg! is a macro that prints the value of the expression
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // to display the struct, we can implement the Debug trait
    println!("rect1 is {:?}", rect1);

    dbg!(&rect1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
