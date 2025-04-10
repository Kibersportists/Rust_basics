fn main() {
    let x = 2; // FILO - First In Last Out
    let y = x; // LIFO - Last In First Out

    example();

    println!("Sumn = {}", add(2, 2));
    // Stack memory: Data is added on top of each other
    
    // Heap memory: We have to search within heap to allocate mmeory that will be enough to store our data
    let string = String::from("Hello"); // This creates a dynamically sized string inside Heap memory
    // Stack will contain a pointer that points towards the string inside the Heap memory
} // Here the scope ends and all information inside it will be deleted in the end

fn example() {
    let a = "true"; // FILO - First In Last Out
    let b = false; // LIFO - Last In First Out
} // Here the scope ends and all information inside it will be deleted in the end

fn add(x: i32, y: i32) -> i32 {
    x + y
}
