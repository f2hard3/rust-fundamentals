// Lifetime annotation

fn choose_favorite_1<'a>(first: &'a str, second: &str) -> &'a str {
    println!("{first} {second}");

    first
}

fn choose_favorite_2<'a>(first: &str, second: &'a str) -> &'a str {
    println!("{first} {second}");

    second
}

fn choose_favorite_3<'a>(first: &str, second: &str) -> &'a str {
    println!("{first} {second}");

    "random"
}

fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let orlando = String::from("Orlando");
    // let result = {
    //     let san_francisco = String::from("San Francisco");

    //     longest(&orlando, &san_francisco) // error
    // };

    println!("{orlando}");
}
