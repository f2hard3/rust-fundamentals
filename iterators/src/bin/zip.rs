use std::iter::zip;

fn main() {
    let first_names = ["Sunggon", "Jigyo", "Youri", "John"];
    let last_names = ["Park", "Park", "Choi"];

    for (first_name, last_name) in zip(first_names.iter(), last_names.iter()) {
        println!("{first_name} {last_name}");
    }

    for (first_name, last_name) in first_names.iter().zip(last_names) {
        println!("{first_name} {last_name}");
    }

    let complete_names: Vec<String> = first_names
        .iter()
        .zip(last_names)
        .map(|(first_name, last_name)| format!("{first_name} {last_name}"))
        .collect();
    println!("{complete_names:?}");
}
