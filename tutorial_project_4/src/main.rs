use std::io;

fn main() {
    // Math operations can not be performed on variables taht are not of the same data type
    let x: i8 = 14; // -128 - 127
    let y: i8 = 1; // -128 - 127

    let z = y - x; // output will be i8 = -13
    println!("{}", z);


    // Remainder caluclation
    let a = 259_i16;
    let b = 10_i16;

    let c = a % b; // output will be i16 = 9
    println!("{}", c);

    // Typecasting
    let d = a / x as i16; // output will be i16 = 18
    println!("{}", d);

    // Input by user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let int_input: i64 = input.trim().parse().unwrap();
    // .trim() trims the '\n' at the end of input string
    // .parse() returns an integer from string if possible
    // .unwrap() extract the value from result, panicks if encounters error
    let e = x as i16 + int_input as i16;
    println!("{} + {} = {}", x, int_input, e);
}
