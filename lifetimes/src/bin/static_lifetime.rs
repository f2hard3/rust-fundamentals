// const COUNT: i32 = 400;

// fn say_hello() -> &'static str {
//     "Hello"
// }

// fn value() -> &'static i32 {
//     &COUNT
// }

// fn main() {
//     let greeting = say_hello();
//     println!("{greeting}");

//     let count = value();
//     println!("{count}");
// }

fn main() {
    let mut fruits = vec!["Apples", "Strawberries", "Pears"];
    let fruit_ref_1 = &mut fruits;
    let fruit_ref_2 = &mut fruits;
    println!("{fruit_ref_2:?}");
}
