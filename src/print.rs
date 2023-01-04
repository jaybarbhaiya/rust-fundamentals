pub fn run() {
    // print statement
    println!("Hello from print.rs");

    // basic formatting
    println!("Hello {} from {}", "Foo", "Bar");

    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Foo", "Bar", "code");

    // Named argurments
    println!("{name} likes to play {activity}", name = "John", activity="Baseball");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}