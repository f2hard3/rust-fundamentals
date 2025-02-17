fn main() {
    // imperative approach
    let numbers = vec![4, 8, 15, 16, 23, 42];

    let mut current_index = 0;
    let final_index = numbers.len() - 1;

    loop {
        if current_index > final_index {
            break;
        }

        println!("{}", numbers[current_index]);
        current_index += 1;
    }

    // more declarative
    current_index = 0;
    while current_index <= final_index {
        println!("{}", numbers[current_index]);
        current_index += 1;
    }

    // more declarative
    for number in numbers {
        println!("{}", number);
    }
}
