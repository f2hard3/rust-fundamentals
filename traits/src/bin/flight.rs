#[derive(Debug, PartialEq, Eq)]
struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}

impl BusTrip {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq<Flight> for BusTrip {
    fn eq(&self, other: &Flight) -> bool {
        self.time == other.time
    }
}

// #[derive(Debug, PartialEq)]
#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }

    // fn ne(&self, other: &Self) -> bool {
    //     !self.eq(other)
    // }
}

impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        self.time == other.time
    }
}

fn main() {
    let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "23:00");
    let c = Flight::new("New York", "Los Angeles", "08:00");
    let d = Flight::new("New York", "London", "08:00");

    println!("{}", a == b);
    println!("{}", a == d);
    println!("{}", a.eq(&b));
    println!("{}", a.ne(&b));
    println!("{}", a != c);
    println!("{}", a == c);

    let a = Flight::new("New York", "London", "08:00");
    let b = BusTrip::new("Los Angeles", "Tokyo", "08:00");
    println!("{}", a == b);
    println!("{}", b == a);
    println!("{}", b == b);

    // Eq trait
    let a = Flight::new("New York", "London", "23:00");
    let b = Flight::new("New York", "London", "23:00");
    let c = Flight::new("New York", "London", "23:00");

    println!("----------Eq trait----------");
    println!("{}", a == a); // reflexive
    println!("{}", a == b); // symmetric // transitive
    println!("{}", b == a); // symmetric
    println!("{}", b == c); // transitive
    println!("{}", a == c); // transitive

    let division = 0.0 / 0.0;
    println!("{}", division);

    let value = 3.4;
    println!("{}", value == value);
    println!("{}", division == division); // Nan is not equal to Nan
                                          // f32, f64 implement PartialEq, not Eq
}
