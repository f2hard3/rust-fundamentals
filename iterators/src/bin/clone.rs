fn main() {
    let teas = [
        String::from("Hot Earl Grey"),
        String::from("Ice Green"),
        String::from("Hot Matcha"),
    ];

    let more_teas: Vec<String> = teas.iter().cloned().collect();
    println!("{more_teas:?}");

    let hot_teas: Vec<String> = teas
        .iter()
        .filter(|tea| tea.contains("Hot"))
        .cloned()
        .collect();
    println!("{hot_teas:?}");
}
