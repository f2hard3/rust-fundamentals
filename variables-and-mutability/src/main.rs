#![allow(unused_variables)]

const TAX_RATE: f64 = 7.25;
const TOUCHDOWN_POINTS: i32 = 6;

type Meters = i32;

// #[allow(unused_variables)]
fn main() {
    let apples = 50;
    let oranges = 14 + 6;
    let _fruits = apples + oranges;

    println!("This year, my garden has {apples} apples and {oranges} oranges.");
    println!(
        "This year, my garden has {} apples and {} oranges.",
        apples, oranges
    );
    println!(
        "This year, my garden has {0} apples and {1} oranges. I can't believe I have {0}",
        apples, oranges
    );

    let mut gym_reps = 10;
    println!("I plan to do {gym_reps} reps");

    gym_reps = 15;
    println!("I plan to do {gym_reps} reps");

    let grams_of_protein = "100.345";
    let grams_of_protein = 100.345;
    #[allow(unused_assignments)]
    let mut grams_of_protein = 100;

    grams_of_protein = 50;
    println!("I plan to intake {grams_of_protein} grams of protein a day.");

    let coffee_price = 5.99;

    {
        println!("The price is {coffee_price}");
        let coffee_price = 1.99;
        println!("The price is {coffee_price}");
    }

    println!("The price is {coffee_price}");

    let income: i32 = 100_000;
    println!("My income is {income} and my tax rate is {TAX_RATE}");

    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
    println!("A one mile race is {mile_race_length} meters long and a two mile race is {two_mile_race_length} meters long.");

    let season: &str = "summer";

    #[allow(unused_assignments)]
    let mut points_scored: i32 = 28;
    points_scored = 35;

    let event_time = "06:00";
    let event_time = 6;

    println!("{points_scored}, {TOUCHDOWN_POINTS}, {event_time}");

    let favorite_beverage = "water";
}
