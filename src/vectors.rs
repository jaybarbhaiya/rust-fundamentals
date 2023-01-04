// Vectors are resizable arrays
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers);

    // get single value
    println!("Single value: {}", numbers[0]);

    // reassing a value
    numbers[2] = 20;

    // Add on to Vector
    numbers.push(7);
    numbers.push(10);
    numbers.pop();

    println!("{:?}", numbers);

    // Array length
    println!("Array length: {}", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number :{}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Mutated numbers: {:?}", numbers);
}