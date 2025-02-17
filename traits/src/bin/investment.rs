trait Investment<T> {
    // fn amount(&self) -> f64;

    // fn set_amount(&mut self, new_amount: f64);

    // fn double_amount(&mut self) {
    //     self.set_amount(self.amount() * 2.0)
    // }

    fn amount(&self) -> T;
    fn double_amount(&mut self);
}

trait Taxable: Investment<f64> {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Investment<f64> for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    // fn set_amount(&mut self, new_amount: f64) {
    //     self.amount = new_amount;
    // }

    fn double_amount(&mut self) {
        self.amount *= 2.0
    }
}

impl Taxable for Income {}

#[derive(Debug)]
struct Bonus {
    value: f64,
}

impl Investment<f64> for Bonus {
    fn amount(&self) -> f64 {
        self.value
    }

    // fn set_amount(&mut self, new_amount: f64) {
    //     self.value = new_amount;
    // }

    fn double_amount(&mut self) {
        self.value *= 2.0
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5;
}

#[derive(Debug)]
struct QualityTime {
    minutes: u32,
}

impl Investment<u32> for QualityTime {
    fn amount(&self) -> u32 {
        self.minutes
    }

    // fn set_amount(&mut self, new_amount: f64) {
    //     self.minutes = new_amount;
    // }

    fn double_amount(&mut self) {
        self.minutes *= 2
    }
}

fn main() {
    let mut income = Income { amount: 50000.50 };
    println!("Income tax owed {:.2}", income.tax_bill());
    income.double_amount();
    println!("Income tax owed {:.2}", income.tax_bill());

    let mut bonus = Bonus { value: 10000.23 };
    println!("Bonus tax owed {:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Bonus tax owed {:.2}", bonus.tax_bill());

    let mut rust_programming_time = QualityTime { minutes: 1000 };
    rust_programming_time.double_amount();
    println!(
        "Rust programming time {:.2}",
        rust_programming_time.amount()
    );
}
