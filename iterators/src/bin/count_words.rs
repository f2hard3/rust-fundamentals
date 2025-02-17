use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<char, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    for word in words {
        for char in word.chars() {
            let count = counts.entry(char).or_insert(0);
            *count += 1;
        }

        // let count = counts.entry(word).or_insert(0);
        // *count += 1;
    }

    counts
}
fn main() {
    println!(
        "{:?}",
        count_words("Sally sells sea shells by the sea shore")
    );
}
