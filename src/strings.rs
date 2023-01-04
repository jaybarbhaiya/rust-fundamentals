// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let hello = "hello";

    let mut hello_heap = String::from("hello_heap ");

    // get length (works for both string types)
    let (len, len_heap) = (hello.len(), hello_heap.len());

    // .push() is not available for type str
    // hello.push('W');

    // .push() is used for String to push a char
    hello_heap.push('W');

    // this will push a string
    hello_heap.push_str("orld");

    // Capacity in bytes
    println!("{}", hello_heap.capacity());

    // check if empty
    println!("Is Empty: {}", hello_heap.is_empty());

    // check for substring
    println!("Contains World: {}", hello_heap.contains("World"));

    // replace
    println!("Replace: {}", hello_heap.replace("World", "There"));

    // loop through string by whitespace
    for word in hello_heap.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{:?}", (hello, hello_heap, len, len_heap, s));
}