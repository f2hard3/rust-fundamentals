fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];
    println!("some: {}", numbers.iter().sum::<i32>());
    println!("product: {}", numbers.iter().product::<i32>());
    println!("max: {}", numbers.iter().max().unwrap());
    println!("min: {}", numbers.iter().min().unwrap());
    println!("count: {}", numbers.iter().count());
    println!("count: {}", numbers.iter().len());
    println!("count: {}", numbers.len());

    let invalid = 0.0 / 0.0;
    println!("{invalid}");

    let numbers = vec![4.6, 8.8, 0.0 / 0.0, 6.2, f64::NAN];
    println!("{numbers:?}");
    let total: f64 = numbers.iter().sum();
    println!("{total}");

    let total: f64 = numbers.iter().filter(|number| !number.is_nan()).sum();
    println!("{total}");

    // let max = numbers.iter().max().unwrap(); // error
    let max = numbers
        .iter()
        // .filter(|number| !number.is_nan())
        .copied()
        .reduce(|acc, curr| acc.max(curr));
    println!("{max:?}");
}
