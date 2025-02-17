use std::io;

fn main() {
    // Concatenation
    let mut full_name = String::from("Sylvester");
    let last_name = String::from("Stallone");
    full_name.push(' ');
    full_name.push_str(&last_name); // &String -> &str
    println!("{full_name}");
    println!("{last_name}");

    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");

    let full_name = first_name + &last_name; // add method behind the scene
    println!("{full_name}");
    // println!("{first_name}"); // error

    // Format macro
    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");

    // let icon = format!("{first_name} {last_name}");
    // let icon = format!("{} {}", first_name, last_name);
    let icon = format!("{0:?} {1:#?} {0} {1}", first_name, last_name);
    println!("{icon}");
    println!("{first_name}");
    println!("{last_name}");

    // String method
    let music_genres = "    Rock, Metal, Country, Rap   ";
    println!("{}", music_genres.trim());
    println!("{music_genres}");
    println!("{}", music_genres.trim_start());
    println!("{music_genres}");
    println!("{}", music_genres.trim_end());
    println!("{music_genres}");

    let mut music_genres = "    Rock, Metal, Country, Rap   ";
    music_genres = music_genres.trim();
    println!("{music_genres}");

    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());
    println!("{}", music_genres.replace("a", "@"));

    // let genres = music_genres.split(", ").collect::<Vec<&str>>();
    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{genres:?}");

    // Collecting use input with read_line method
    let mut name = String::new();
    println!("What is your name?");
    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Hello, {}", name.trim()),
        Err(message) => println!("Error: {message}"),
    }
}
