use std::collections::HashSet;

#[derive(Debug)]
struct PlayList {
    songs: Vec<String>,
    users: HashSet<String>,
}

impl FromIterator<(String, String)> for PlayList {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();
        for (song, user) in iter {
            songs.push(song);
            users.insert(user);
        }

        Self { songs, users }
    }
}

fn main() {
    let fifty_numbers = 1..=50;
    let result = fifty_numbers.clone().collect::<Vec<i32>>();
    println!("{result:?}");
    let result = Vec::from_iter(fifty_numbers.clone());
    println!("{result:?}");

    let unique_set: HashSet<_> = fifty_numbers.clone().collect::<HashSet<i32>>();
    println!("{unique_set:?}");
    let unique_set: HashSet<_> = HashSet::from_iter(fifty_numbers.clone());
    println!("{unique_set:?}");

    let chars = ['H', 'e', 'l', 'l', 'o'];
    let greeting = String::from_iter(chars);
    println!("{greeting:?}");

    let songs = [
        (String::from("I Rust Go On"), String::from("Bob")),
        (String::from("A Rust of Wind"), String::from("Bob")),
        (String::from("A Rustworthy Man"), String::from("Sheila")),
    ];

    // let playlist = songs.into_iter().collect::<PlayList>();
    // println!("{playlist:?}");
    let playlist = PlayList::from_iter(songs);
    println!("{playlist:?}");
}
