#[allow(unused_variables)]
fn main() {
    let result = square(5);
    println!("result: {result}");

    let result = mystery();

    let multiplier = 3;
    let calculation = {
        let value = 5 + 4;
        value * multiplier
    };
    println!("calculation: {calculation}");
}

fn square(number: i32) -> i32 {
    // return number * number;
    number * number // implicit return values
}

fn mystery() {
    println!("Hello, there!");
}
