struct Vehicle<'a> {
    name: &'a str,
}

impl<'a> Vehicle<'a> {
    // fn send_transmission(self, msg: &'a str) -> &str {
    //     println!("Sending message: {} to Vehicle: {}", msg, self.name);
    //     self.name
    // }

    fn send_transmission(self, msg: &'a str) -> &str {
        println!("Sending message... {}", self.name);
        msg
    }
}

pub fn run() {
    let car = Vehicle { name: "BMW" };
    println!("{}", car.send_transmission("Driving..."));
}
