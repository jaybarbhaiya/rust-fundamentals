// Conditions - Used to check the conditions of something and act on it

pub fn run() {
    let (age, check_id, knows_person_of_age) = (30, true, true);

    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: what would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry you have to leave");
    } else {
        println!("Bartender: I will need some id");
    }

    // no Ternary operator in Rust
    // there is a short hand if
    let is_age_of = if age >= 21 {true} else {false};
    println!("Is age of: {}", is_age_of);
}