/*
    Primitive types--
    - Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
    - Floats: f32, f64
    - Boolean (bool)
    - Character (char)
    - Tuples
    - Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time,
// however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    // By default it will be i32
    let x = 1;

    // By default it will be f64
    let y = 2.5;

    // add explicit type
    let z: i64 = 23423424;

    // find Max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // the convention to defined variable names is snake case (_) not camel case
    let is_active = true;

    let is_greater = 10 < 5;

    // char must be single quotes
    let a = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a, face));


}