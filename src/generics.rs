pub fn run() {
    let list = vec![234, 435, 123, 453, 64];
    let largest = get_largest(list);
    println!("Largest number: {}", largest);

    let list = vec!['v', 'r', 'e'];
    let largest = get_largest(list);
    println!("Largest character: {}", largest);

    let p1 = Point { x: 10, y: 10 };
    println!("{} {}", p1.x, p1.y);

    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// implementing generics in struct
struct Point<T> {
    x: T,
    y: T,
}

// Generics in impl block is not bound to the generics,
// used above, but as a general practice is good to result the defined
// rederences to make the code readability better
impl<J> Point<J> {
    fn x(&self) -> &J {
        &self.x
    }
}
