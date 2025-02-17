use std::iter::Cycle;

#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug)]
struct Card {
    rank: String,
    suit: CardSuit,
}

#[derive(Debug)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug)]
enum PaymentMethodType {
    Cash,
    CreditCard(String),
    DebitCard(String, String),
    // PayPal(Credentials),
    PayPal { username: String, password: String },
}

// Nesting enums in enums
#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl { meat: Meat, beans: Beans },
    VeganPlate,
}

// Match keyword
#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

#[derive(Debug)]
enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

// Defining methods on enums
impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("cold")
            }
            LaundryCycle::Hot { temperature } => {
                println!("hot temperature: {temperature}")
            }
            LaundryCycle::Delicate(fabric_type) => {
                println!("delicate fabric type: {fabric_type}")
            }
        }
    }
}

#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            // OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
            //     println!("being prepped");
            // }
            OnlineOrderStatus::Delivered => {
                println!("delivered");
            }
            // _ => {
            //     println!("not yet");
            // }
            other_status => {
                println!("{:?}", other_status)
            }
        }
    }
}

#[derive(Debug)]
enum Milk {
    LowFat(i32),
    Whole,
    NonDairy { kind: String },
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::LowFat(2) => {
                println!("2% low fat")
            }
            Milk::LowFat(percent) => {
                println!("{percent}% low fat")
            }
            Milk::Whole => {
                println!("whole milk")
            }
            _ => {
                println!("Non Dairy")
            }
        }
    }
}

// if let construct

fn main() {
    let first_card = CardSuit::Hearts;
    let mut second_card = CardSuit::Spades;
    second_card = CardSuit::Clubs;
    println!("{:?}", second_card);

    let card_suits = [CardSuit::Hearts, CardSuit::Clubs];
    println!("{:?}", card_suits);
    let card_suits = (CardSuit::Hearts, CardSuit::Spades);
    println!("{:?}", card_suits);

    // Enum with associated values
    // Memory allocation for any variants: largest possible required size
    let mut payment_method = PaymentMethodType::CreditCard(String::from("0034-5678"));
    payment_method =
        PaymentMethodType::DebitCard(String::from("5451-5610"), String::from("f2hard3@gmail.com"));
    println!("{:?}", payment_method);

    // Struct variants
    // let paypal_credentials = Credentials {
    //     username: String::from("f2hard3@gmail.com"),
    //     password: String::from("password"),
    // };
    // let paypal = PaymentMethodType::PayPal(paypal_credentials);

    let visa = PaymentMethodType::CreditCard(String::from("0012-3456"));
    let paypal = PaymentMethodType::PayPal {
        username: String::from("f2hard3@gmail.com"),
        password: String::from("password"),
    };
    println!("{:?}", visa);
    println!("{:?}", paypal);

    // Nesting enums in enums
    let lunch = RestaurantItem::Burrito {
        meat: Meat::Chicken,
        beans: Beans::Pinto,
    };
    let dinner = RestaurantItem::Bowl {
        meat: Meat::Steak,
        beans: Beans::Black,
    };
    let abandoned_meal = RestaurantItem::VeganPlate;

    println!("Lunch was {:?} and dinner was {:?}", lunch, dinner);
    println!("Nobody ate {:?}", abandoned_meal);

    // Match keyword
    let laptop = OperatingSystem::MacOS;
    println!("{}", years_since_release(&laptop));
    let desktop = OperatingSystem::Windows;
    println!("{}", years_since_release(&desktop));

    wash_laundry(LaundryCycle::Cold);
    wash_laundry(LaundryCycle::Hot { temperature: 45 });
    wash_laundry(LaundryCycle::Delicate(String::from("Sweater")));

    // Defining methods on enums
    LaundryCycle::Cold.wash_laundry();
    let hot_cycle = LaundryCycle::Hot { temperature: 72 };
    hot_cycle.wash_laundry();
    let delicate_cycle = LaundryCycle::Delicate(String::from("Silk"));
    delicate_cycle.wash_laundry();

    OnlineOrderStatus::Delivered.check();
    OnlineOrderStatus::Packed.check();
    OnlineOrderStatus::Shipped.check();

    Milk::LowFat(1).drink();
    Milk::LowFat(2).drink();
    Milk::Whole.drink();

    // if let construct
    let milk = Milk::LowFat(5);
    if let Milk::LowFat(percent) = milk {
        println!("You have {percent}% low fat milk")
    }

    let milk = Milk::NonDairy {
        kind: String::from("juice"),
    };
    if let Milk::NonDairy { ref kind } = milk {
        println!("You have {kind}")
    } else {
        println!("You have other milk variant")
    }
    if let Milk::NonDairy { kind } = &milk {
        println!("You have {kind}")
    } else {
        println!("You have other milk variant")
    }
    println!("{milk:?}");

    // let else construct
    // let beverage = Milk::Whole;
    // let Milk::LowFat(percent) = beverage else {
    //     println!("You do not have low fat milk.");
    //     return;
    // };
    // println!("{percent}% milk is available");

    let beverage = Milk::NonDairy {
        kind: String::from("orange juice"),
    };
    let Milk::NonDairy { kind } = beverage else {
        println!("This is milk.");
        return;
    };
    println!("This is not milk but {kind}");
}

fn years_since_release(os: &OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Quite an old operating system");
            39
        }
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }
}

fn wash_laundry(cycle: LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => {
            println!("cold")
        }
        LaundryCycle::Hot { temperature } => {
            println!("hot temperature: {temperature}")
        }
        LaundryCycle::Delicate(fabric_type) => {
            println!("delicate fabric type: {fabric_type}")
        }
    }
}
