#[derive(Debug)]
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    // Associated function: a method without self keyword as a parameter
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        TaylorSwiftSong {
            title,
            release_year,
            duration_secs,
        }
    }
}

impl TaylorSwiftSong {
    fn display_song_info(&self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Years since release: {}", self.years_since_release());
        println!("Duration: {} seconds", self.duration_secs);
    }

    fn double_song(&mut self) {
        self.duration_secs *= 2;
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn years_since_release(&self) -> u32 {
        2025 - self.release_year
    }
}

// Builder Pattern
#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }
}

impl Computer {
    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}

// Tuple structs
struct ShortDuration(u32, u32); // hours, minutes
struct LongDuration(u32, u32); // years, months

// Unit structs
struct Empty;

fn main() {
    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 2.99,
        is_hot: true,
    };

    println!(
        "My {} this morning cost {}. It is {} that is was hot.",
        mocha.name, mocha.price, mocha.is_hot
    );

    let favorite_coffee = mocha.name;
    println!("{favorite_coffee}");
    // println!("{}", mocha.name); // error

    let mocha_price = mocha.price;
    println!("{mocha_price}");
    println!("{}", mocha.price);

    // Overwrite struct fields
    // There is no partial mutation
    let mut beverage = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };

    beverage.name = String::from("Americano");
    beverage.price = 6.99;
    beverage.is_hot = true;
    println!(
        "My {} this morning cost {}. It is {} that is was hot.",
        beverage.name, beverage.price, beverage.is_hot
    );

    let name = String::from("Latte");
    let coffee = make_coffee(name, 4.99, true);
    println!(
        "My {} this morning cost {}. It is {} that is was hot.",
        coffee.name, coffee.price, coffee.is_hot
    );

    let caramel_macchiato = Coffee {
        name: String::from("Caramel Macchiato"),
        price: mocha.price,
        ..coffee
    };
    println!(
        "My {} this morning cost {}. It is {} that is was hot.",
        caramel_macchiato.name, caramel_macchiato.price, caramel_macchiato.is_hot
    );

    let mut new_coffee = Coffee {
        name: coffee.name.clone(),
        ..coffee
    };
    println!(
        "My {} this morning cost {}. It is {} that is was hot.",
        coffee.name, coffee.price, coffee.is_hot
    );
    println!(
        "My {} this morning cost {}. It is {} that is was hot.",
        new_coffee.name, new_coffee.price, new_coffee.is_hot
    );

    drink_coffee(&mut new_coffee);
    println!("{:?}", new_coffee);
    println!("{:#?}", new_coffee);

    // Struct methods
    let mut blank_space = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    blank_space.display_song_info();
    blank_space.double_song();
    blank_space.display_song_info();

    let all_too_well = TaylorSwiftSong {
        title: String::from("All Too Well"),
        release_year: 2012,
        duration_secs: 327,
    };

    if blank_space.is_longer_than(&all_too_well) {
        println!(
            "{} is longer than {}",
            blank_space.title, all_too_well.title
        );
    } else {
        println!(
            "{} is short than or equal to {}",
            blank_space.title, all_too_well.title
        );
    }

    // Associated function
    let new_song = TaylorSwiftSong::new(String::from("New song"), 2025, 498);
    new_song.display_song_info();

    // Builder pattern
    let mut computer = Computer::new(String::from("M3 Max"), 64, 2);
    computer
        .upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(128)
        .upgrade_hard_drive_capacity(4);

    println!("Stats: {computer:#?}");

    // Tuple structs
    let work_shift = ShortDuration(8, 0);
    go_to_work(work_shift);

    let era = LongDuration(5, 3);
    // go_to_work(era); // error
    println!("{} years {} months", era.0, era.1);

    // Unit-like structs
    let my_empty_struct = Empty;
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}

fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my delicious {}", coffee.name);
    coffee.is_hot = false;
    coffee.price = 10.99;
}

fn go_to_work(length: ShortDuration) {
    let work_shift = ShortDuration(8, 0);
    println!("{} hours {} minutes", work_shift.0, work_shift.1);
}
