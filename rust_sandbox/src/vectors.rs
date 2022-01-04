// Resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Add
    numbers.push(5);
    numbers.push(6);

    // Pop last val
    numbers.pop();
    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {:?}", numbers[2]);

    // Get vector length
    println!("Vector length: {:?}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}
