// Arrays - fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Reassign values
    numbers[2] = 20;
    
    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get the array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers;

    println!("Slice: {:?}", slice);
}