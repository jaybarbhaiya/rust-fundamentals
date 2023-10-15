use std::fmt;

struct Person {
    name: String,
    age: u32,
}

// YOUR CODE GOES HERE
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, age: {}", self.name, self.age)
    }
}

pub fn run() {
    let john = Person {
        name: String::from("John"),
        age: 22,
    };
    println!("Person entered is {}", john);
}
