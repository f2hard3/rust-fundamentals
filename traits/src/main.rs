use std::ops::Add;

#[derive(Debug)]
struct Lunch {
    cost: f64,
}

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

impl Add for Lunch {
    // associated type that belongs to Add trait
    type Output = Lunch;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
        }
    }
}

fn main() {
    let one = Lunch { cost: 19.99 };
    let two = Lunch { cost: 29.99 };
    println!("{:.2?}", one + two);

    let integer_sum = add_two_numbers(1, 2);
    println!("{:.2?}", integer_sum);
    let float_sum = add_two_numbers(1.05, 2.75);
    println!("{:.2?}", float_sum);
}
