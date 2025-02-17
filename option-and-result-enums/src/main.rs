// Building option from scratch
#[derive(Debug, Copy, Clone)]
enum CustomOption<T> {
    Some(T),
    None,
}

impl<T> CustomOption<T> {
    fn unwrap(self) -> T {
        match self {
            CustomOption::Some(value) => value,
            CustomOption::None => panic!("Error"),
        }
    }

    fn unwrap_or(self, default: T) -> T {
        match self {
            CustomOption::Some(value) => value,
            CustomOption::None => default,
        }
    }
}

#[allow(unused_variables)]
fn main() {
    // Option Enum
    let a = Option::Some(5);
    let b = Option::Some("hello");
    let c = Option::Some("true");

    let a: Option<i8> = Option::Some(5);
    let b = Option::<i16>::Some(5);

    let d: Option<&str> = Option::None;
    let d = Option::<&str>::None;

    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    // Unwrap and expect method
    let bass = musical_instruments.get(2);
    println!("{:?}", bass);
    let valid_instrument = bass.unwrap();
    println!("{:?}", valid_instrument);
    let valid_instrument = bass.expect("Unable to retrieve element");
    println!("{:?}", valid_instrument);

    let invalid_instrument = musical_instruments.get(100);
    println!("{:?}", invalid_instrument);
    // let valid_instrument = invalid_instrument.unwrap(); // runtime error
    // let invalid_instrument = invalid_instrument.expect("Unable to retrieve musical instrument"); // runtime error
    // println!("{:?}", invalid_instrument);

    // Match keyword with option enums
    play(bass);
    println!("{:?}", bass);
    play(invalid_instrument);

    // Returning option enum
    let availability = is_item_in_stock(true, true);
    match availability {
        Some(value) => println!("Item is available: {value}"),
        None => println!("Your item doesn't exist in our system"),
    }

    match availability {
        Some(true) => println!("Item is available"),
        Some(false) => println!("Item is not available"),
        None => println!("Your item doesn't exist in our system"),
    }

    // Unwrap_or method
    let present_value = Some(13);
    let missing_value: Option<i32> = None;

    println!("{}", present_value.unwrap_or(0));
    println!("{}", missing_value.unwrap_or(0));

    // Building option from scratch
    let some_option = CustomOption::Some(100);
    println!("{}", some_option.unwrap());
    println!("{}", some_option.unwrap_or(0));
    let none_option: CustomOption<i32> = CustomOption::None;
    // println!("{}", none_option.unwrap());
    println!("{}", none_option.unwrap_or(0));

    // Result enum
    let ok: Result<i32, &str> = Ok(5);
    println!("{:?}", ok);
    let disaster: Result<i32, &str> = Err("Something went wrong");
    println!("{:?}", disaster);

    let text = "50";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number);

    let text = "Seoul";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number);

    let result = divide(10.0, 0.0);

    // match result {
    //     Ok(calculation) => println!("{:?}", calculation),
    //     Err(message) => println!("{:?}", message),
    // }

    // println!("{:?}", result.unwrap());
    // println!("{:?}", result.expect("Unable to parse calculation"));
    println!("{:?}", result.is_ok());
    println!("{:?}", result.is_err());
    println!("{:?}", result.unwrap_or(0.0));

    // Nuance of unwrap method
    let my_result = operation(true);
    let content = match my_result {
        Ok(message) => message,
        Err(error_message) => error_message,
    };
    println!("{:?}", my_result.unwrap());
    println!("{:?}", my_result.unwrap());
}

fn play(instrument_option: Option<&String>) {
    // Option implements copy trait. so it does not transfer the ownership
    match instrument_option {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }
}

fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_stock && item_is_in_stock {
        Some(true)
    } else if item_is_in_system {
        Some(false)
    } else {
        None
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn operation(great_success: bool) -> Result<&'static str, &'static str> {
    if great_success {
        Ok("Success")
    } else {
        Err("Error")
    }
}
