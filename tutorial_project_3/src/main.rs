use std::io; // standard crate with input/output module
// '::' is the path seperator operator

fn main() {
    println!("Hello, world!");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");
    // Calling a variable in Rust by default creates a copy of it, thats why the reference '&mut' is required
    // If expect wouldn't be entered there would be a warning that an unused 'Result' must be used
    // read_line() requires a string input, so other input data types will raise an error

    println!("{}", input);
}
