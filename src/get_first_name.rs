pub fn run() {
    let input_string = "Hello world";
    let split_input_vec: Vec<&str> = input_string.split(" ").collect();
    println!("{}", split_input_vec[0])
}
