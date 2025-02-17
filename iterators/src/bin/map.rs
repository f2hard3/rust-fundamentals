use std::collections::HashSet;

fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    for number in numbers.into_iter().map(|number: i32| number.pow(2)) {
        println!("square: {number}")
    }
    // println!("{numbers:?}"); // error

    let numbers = vec![4, 4, 8, 15, 16, 23, 42];
    // let squares: Vec<_> = numbers.iter().map(|number: &i32| number.pow(2)).collect();
    // let squares: Vec<i32> = numbers.iter().map(|number: &i32| number.pow(2)).collect();
    let squares = numbers
        .iter()
        .map(|number: &i32| number.pow(2))
        .collect::<Vec<i32>>();
    println!("{:?}", squares);
    println!("{:?}", numbers);

    // let squares: HashSet<i32> = numbers.iter().map(|number: &i32| number.pow(2)).collect();
    let squares = numbers
        .iter()
        .map(|number: &i32| number.pow(2))
        .collect::<HashSet<i32>>();
    println!("{:?}", squares);
    println!("{:?}", numbers);

    let names = [
        String::from("Sunggon"),
        String::from("Jigyo"),
        String::from("Youri"),
    ];
    let name_lengths: Vec<usize> = names
        .iter()
        .map(|name| name.to_lowercase())
        .map(|name| name.replace("i", "@@"))
        .map(|name| name.len())
        .collect();
    println!("{name_lengths:?}");

    let name_lengths: Vec<usize> = names
        .iter()
        .map(|name| name.to_lowercase().replace("i", "@@").len())
        .collect();
    println!("{name_lengths:?}");
}
