enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


pub fn main() {
    // empty vector
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    // pre-filled
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // getting an specific element
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // matching an specific element if exists
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; -> panic
    let does_not_exist = v.get(100);
    print!("{:?}", does_not_exist);

    // iterating a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // creating a vector with an enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
