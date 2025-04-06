fn main() {
    // The following variables will raise warnings if they are not used anywhere else in the code
    let integer = 2; // This will be signed integer 32 bit (i32)
    let floating_point = 10.9; // This will be 64 bit float (f64)
    let true_or_false = false; // true == 1; false == 0
    let letter = ';'; // char datatype

    // Tuple - fixed length sequence of elements that is immutable
    let mut tup: (i32, bool, char) = (1, true, 's');
    tup = (10, false, 'a'); // This will raise a warning, since arr[4] value was not used between its initialization and redefinition
    println!("{}", tup.2);

    // Arrays will have the default datatype for i32. Their length is immutable and datatype the same for all indexes
    let mut arr: [i16; 5] = [1, 2, 3, 4, 5]; // The [i16; 5] indicates the variable datatypes and the variable count inside the array
    arr[4] = 3; // This will raise a warning, since arr[4] value was not used between its initialization and redefinition
    println!("{}", arr[4]);

    // Be careful with variable types
    let x: u8 = 4;
    let y: i32 = x as i32; // x has to be converted to the i32 datatype before assigning it to y
    println!("{}, {}", x, y);
}
