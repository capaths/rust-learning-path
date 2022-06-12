const LINES: [&str; 12] = [
    "partridge in a pear tree",
    "Two turtle-doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings (five golden rings)",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "12 drummers drumming"
];

const NUMERALS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];

fn main() {
    let mut line_index: usize = 0;
    let n_lines: usize = LINES.len();
    while line_index < n_lines {
        println!("On the {} day of Christmas my true love sent to me", NUMERALS[line_index]);
        for i in (0..(line_index + 1)).rev() {
            if i == 0 {
                if line_index == 0 {
                    println!("A {}", LINES[i]);
                } else {
                    println!("And a {}", LINES[i]);
                }
            } else {
                println!("{}", LINES[i]);
            }
        }
        line_index += 1;
        println!("");
    }
}

