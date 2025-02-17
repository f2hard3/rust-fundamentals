fn main() {
    let earnings = [4, 7, 9, 13];
    let sum = earnings.into_iter().reduce(|acc, num| acc + num);
    println!("{:?}", sum);

    let earnings: [i32; 0] = [];
    let sum = earnings.into_iter().reduce(|acc, num| acc + num);
    println!("{:?}", sum);

    let address_portions = [
        String::from("Uenodai 1-3-18-601"),
        String::from("Fujimino"),
        String::from("Saitama"),
    ];
    println!("{}", address_portions.join(", "));

    let address = address_portions.into_iter().reduce(|mut acc, portion| {
        acc.push_str(", ");
        acc.push_str(&portion);
        acc
    });
    println!("{:?}", address);
}
