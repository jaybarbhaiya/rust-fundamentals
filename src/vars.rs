// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scope language

pub fn run() {
    let name = "Foo";

    // mutable variable
    let mut age = 30;

    println!("My name is {} and I am {} years old", name, age);
    age = 40;
    println!("New age is {} years old", age);

    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Foo", 30);
    println!("{} is {}", my_name, my_age);
}