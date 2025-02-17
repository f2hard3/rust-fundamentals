use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<char, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    words.for_each(|word| {
        word.chars().for_each(|char| {
            let count = counts.entry(char).or_insert(0);
            *count += 1;
        });
    });

    counts
}

fn main() {
    println!(
        "{:?}",
        count_words("Sally sells sea shells by the sea shore")
    );
}
