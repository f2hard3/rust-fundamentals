fn main() {
    let multiplier = 5;

    // fn multiply_by(value: i32) -> i32 {
    //     value * multiplier // error
    // }

    let multiply_by = |value| value * multiplier;
    println!("{}", multiply_by(5 as u8));
    // println!("{}", multiply_by(1000)); // error

    let product = |a: i32, b: i32| -> i32 {
        println!("Calculating product for you");
        a * b
    };
    println!("{}", product(3, 10));
    println!("{}", product(5, 8));

    let mirror = |value| value;
    println!("{}", mirror("abc"));
    // println!("{}", mirror(10)); // error
}
