fn main() {
    println!("Hello, world!");
    add_numbers(20, 32);

    // Functions in Rust can return experssions, but NOT statements
    let number = {
        let x = 3;
        x + 1 // Addition of ';' would make this a statement and return and error
    }; // The {} makes it an experssion
    println!("number = {}", number); // Output: number = 4

    let result = add_numbers_2(2, 3);
    println!("Result = {}", result);
}

fn add_numbers(x: i32, y: i32) {
    println!("The sum is: {}", x + y);
}

fn add_numbers_2(x: i32, y: i32) -> i32{ // the '->' is the return operator in Rust
    x + y // Adding a ';' would make it a statement and cause an error. 
    // Another option would be to write 'return x + y;', since then it would be a statement that is supposed to return an experssion
}


