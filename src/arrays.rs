// Arrays are fixed length, same data type
// Look at vectors for growability

// Without  use std::mem  you must write  std::mem::size_of_val()
// With     use std::mem  you can write        mem::size_of_val()
use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Will error because it's expecting 5 elements.
    // let numbers: [i32; 5] = [1, 2, 3, 4];

    // Will error because it's expecting i32s.
    // let numbers: [i32; 5] = ['1', '2', '3', '4', '5'];

    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Reassign values
    let mut mutable_numbers: [i32; 5] = [1, 2, 3, 4, 5];

    mutable_numbers[2] = 20;

    println!("Reassigned value: {}", mutable_numbers[2]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack-allocated
    println!("Array occupies {} bytes.", mem::size_of_val(&numbers));

    // Slice of an array
    let slice: &[i32] = &numbers[0..2];
    println!("Slice (index 0-2): {:?}", slice);

    let other_slice: &[i32] = &numbers[2..4];
    println!("Slice (index 2-4): {:?}", other_slice);
}
