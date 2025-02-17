use std::fmt::{Debug, Display, Formatter, Result};
use std::fs;
use std::ops::Drop;

enum AppleType {
    RedDelicious,
    GrannySmith,
}

impl Display for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::RedDelicious => write!(f, "🍎 Delicious 🍎"),
            Self::GrannySmith => write!(f, "🍏 Granny Smith 🍏"),
        }
    }
}

impl Debug for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::RedDelicious => write!(f, "AppleType::RedDelicious"),
            Self::GrannySmith => write!(f, "AppleType::GrannySmith"),
        }
    }
}

struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} for {}", self.kind, self.price)
    }
}

impl Debug for Apple {
    // fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    //     write!(
    //         f,
    //         "Apple ::: [ Kind: {:?}, Price: {} ]",
    //         self.kind, self.price
    //     )
    // }
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("** Apple **")
            .field("Kind", &self.kind)
            .field("Price", &self.price)
            .finish()
    }
}

impl Drop for Apple {
    fn drop(&mut self) {
        println!("Apple {} is being cleaned up", self);
        match fs::remove_file("apple.txt") {
            Ok(_) => println!("Goodbye, my sweet apple"),
            Err(error) => eprintln!("Error deleting file: {error}"),
        }
    }
}

fn main() {
    let lunch_snack = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04,
    };

    let dinner_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 1.15,
    };

    println!("{}", lunch_snack);
    println!("{}", dinner_snack);

    println!("{:?}", lunch_snack);
    println!("{:?}", dinner_snack);
}
