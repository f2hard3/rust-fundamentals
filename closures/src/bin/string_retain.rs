fn main() {
    let mut game_console = String::from("PlayStation");
    // let closure = |character| character != 'a';

    let mut deleted_characters = String::new();
    let closure = |character| {
        if character != 'a' {
            true
        } else {
            deleted_characters.push(character);
            false
        }
    };

    game_console.retain(closure);
    println!("{game_console}");
    println!("{deleted_characters}");
}
