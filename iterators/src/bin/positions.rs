fn main() {
    let performers = ["Rustful Five", "Rust in Peace", "Rustin Bieber"];

    let last = performers.iter().last().unwrap();
    println!("{last}");

    let second = performers.iter().nth(1).unwrap();
    println!("{second}");

    let second_to_last = performers.iter().nth_back(1).unwrap();
    println!("{second_to_last}");

    let target_index = performers
        .iter()
        .position(|element| *element == "Rustin Bieber")
        .unwrap();
    println!("target_index: {target_index}");
}
