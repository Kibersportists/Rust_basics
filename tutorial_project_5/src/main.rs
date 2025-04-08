fn main() {
    // Comparison operators in Rust: <, >, <=, >=, !=, ==
    let cond = (2 as f32) < 3.0; // Output: true
    println!("{}", cond);

    // Logical operators in Rust: &&, ||, !
    let cond_2 = false && cond; // Output: false
    println!("{}", cond_2);

    // if, else if, else in Rust
    let food = "fruit";
    if food == "cookie" {
        println!("I like cookies!");
    } else if food == "fruit" {
        println!("I like fruits!");
    } else {
        println!("I don't like anything!");
    }
}
