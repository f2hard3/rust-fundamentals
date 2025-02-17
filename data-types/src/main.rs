#![allow(unused_variables)]
fn main() {
    // Integers
    let eight_bit: i8 = -112;
    // let eight_bit_unsigned: u8 = -15;

    let some_value = 20i8;

    let days: usize = 55;
    let year: isize = -15_000;

    // Strings
    println!("Dear Sunggon,\nHow have you been?");
    println!("\tOnce upon a time");
    println!("Sunggon said \"I love you\"");
    let filepath = "C:\\My Documents\\net\\videos";
    println!("{filepath}");
    let filepath = r"C:\My Documents\net\videos";
    println!("{filepath}");

    // Method
    let value: i32 = -15;
    println!("{}", value.abs());
    println!("{}", value.pow(2));

    let empty_space = "     my content      ";
    println!("{}", empty_space.trim());

    // Float with format specifier
    let pi: f64 = 3.1415926535897932384;
    println!("The current value of pi is {pi:.2}");
    println!("The current value of pi is {:.4}", pi);

    // Char
    let first_initial = 'B';
    let emoji = '😕';
    let example = "B";
    println!(
        "{} {}",
        first_initial.is_alphabetic(),
        emoji.is_alphabetic()
    );
    println!("{} {}", first_initial.is_uppercase(), emoji.is_uppercase());

    // Array
    let numbers: [i32; 6] = [4, 8, 15, 16, 23, 42];
    let apples = ["Granny Smith", "McIntosh", "Red Delicious"];
    println!("Length: {}", apples.len());

    let currency_rates: [f64; 0] = [];

    let mut seasons = ["Spring", "Summer", "Fall", "Winter"];
    // println!("{}", seasons[100]); // compile time error
    seasons[2] = "autumn";
    println!("{:?}", seasons);
    println!("{seasons:?}");
    println!("{seasons:#?}");

    // debug macro
    dbg!(seasons);
    dbg!(2 + 2);

    // Tuple type
    let employee = ("Molly", 32, "Marketing", true, 3.14);
    // let name = employee.0;
    // let age = employee.1;
    // let department = employee.2;
    let (name, age, department, _, _) = employee;
    println!("Name: {name}, age: {age}, department: {department}");
    println!("{employee:?}");
    println!("{employee:#?}");
    dbg!(employee);

    // Range type
    let month_days: std::ops::Range<i32> = 1..31;
    dbg!(month_days);
    let month_days = 1..=31;
    dbg!(&month_days);

    for number in month_days {
        println!("{number}");
    }

    let letters: std::ops::Range<char> = 'b'..'f';
    for letter in letters {
        println!("{letter}");
    }

    let colors = ["Red", "Green", "Blue"];
    for color in colors {
        println!("{color}");
    }
}
