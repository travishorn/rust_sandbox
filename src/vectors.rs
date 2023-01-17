// Vectors are like arrays, but resizable

use std::mem;

pub fn main() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single value (same as array)
    println!("Single value: {}", numbers[0]);

    // Reassign values (same as array)
    numbers[2] = 20;

    println!("Reassigned value: {}", numbers[2]);

    // Push into vector (add value)
    numbers.push(6);
    numbers.push(7);

    println!("After pushes: {:?}", numbers);

    // Pop (remove last value)
    numbers.pop();

    println!("After pop: {:?}", numbers);

    // Get vector length (same as array)
    println!("Vector length: {}", numbers.len());

    // Vectors are stack-allocated (same as array)
    println!("Vector occupies {} bytes.", mem::size_of_val(&numbers));

    // Slice of an vector (same as array)
    let slice: &[i32] = &numbers[0..2];
    println!("Slice (index 0-2): {:?}", slice);

    let other_slice: &[i32] = &numbers[2..4];
    println!("Slice (index 2-4): {:?}", other_slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Vector value: {}", x);
    }

    // Loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Doubled: {:?}", numbers);
}
