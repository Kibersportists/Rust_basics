fn main() {
    let x = 4; // Variables are immutable by default
    // To create a mutable x you have to write: let mut x = 4;
    println!("x is: {}", x); // Output -> x is: 4

    {
        let x = x - 3; // Variable shdow will occur if a variable is redefined in a different scope
        println!("x is: {}", x); // Output -> x is: 3
    }

    let x = x + 1; // Immutable variables can be changed by redefning them
    println!("x is: {}", x); // Output -> x is: 5

    const SECONDS_IN_MINUTE: u32 = 60; // const implies that a variable can't be changed or redefined, otherwise there will be an error
    println!("{}", SECONDS_IN_MINUTE); // Output -> 60
}
