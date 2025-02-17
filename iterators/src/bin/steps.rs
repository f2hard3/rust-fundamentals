fn main() {
    let fifty_numbers: Vec<i32> = (1..50).collect();
    println!("fifty_numbers: {fifty_numbers:?}");
    let fifty_numbers = 1..=50;
    for number in fifty_numbers.rev().take(15).skip(5).step_by(3) {
        print!("{number}/");
    }
    // println!("fifty_numbers: {fifty_numbers:?}"); // error
}
