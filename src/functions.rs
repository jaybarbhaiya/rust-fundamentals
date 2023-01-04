// Functions - used to store block of code for resue

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // do not put a ; when you want to return a value from the function
    n1 + n2
}

pub fn run() {
    greeting("Hello", "FooBar");

    // bind function values to variables
    let sum = add(5,5);
    println!("Sum: {}", sum);

    // Closure
    // the advantage of Closures is that we can use the external values which are outside the scope of a function.
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(3, 3));
}