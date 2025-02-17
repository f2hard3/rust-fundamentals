fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}");
}

fn main() {
    // Assigning result of if statement to variable
    even_or_odd(17);

    // Match statement
    let evaluation = false;
    let val = match evaluation {
        // true => {
        //     println!("The val is true");
        // }
        // false => {
        //     println!("The val is false")
        // }
        true => 20,
        false => 40,
    };

    println!("{val}");

    let season = "spring";
    match season {
        // "summer" => {
        //     println!("School's out!");
        // }
        // "winter" => {
        //     println!("Brr, so cold!");
        // }
        // _ => {
        //     println!("Lot's of rain!")
        // }
        "summer" => println!("School's out!"),
        "winter" => println!("Brr, so cold!"),
        _ => println!("Lot's of rain!"),
    }

    let number = 7;
    match number {
        // 2 | 4 | 6 | 8 => println!("{number} is even"),
        // 1 | 3 | 5 => println!("{number} is odd"),
        // _ => println!("Unknown for now"),
        value if value % 2 == 0 => println!("{value} is an even number"),
        x if x % 2 != 0 => println!("{x} is an odd number"),
        _ => unreachable!(),
    }

    // Loop
    let mut seconds = 21;
    loop {
        if seconds <= 0 {
            println!("Blastoff!");
            break;
        }

        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds..");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to blastoff..");
        seconds -= 1;
    }

    // While loop
    let mut seconds = 31;
    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds...");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
    }
    println!("Blastoff!");

    // Recursion
    countdown(5);
}

fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("Blastoff!");
    } else {
        println!("{seconds} seconds to blastoff..");
        countdown(seconds - 1);
    }
}
