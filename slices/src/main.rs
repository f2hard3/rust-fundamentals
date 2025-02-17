#[allow(unused_variables)]

fn main() {
    // String slice
    let action_hero = "Arnold Schwarzenegger"; // &str: ref str

    let first_name = &action_hero[..6];
    println!("first name: {first_name}");

    let last_name = &action_hero[7..];
    println!("last name: {last_name}");

    let first_name = {
        let action_hero = "Arnold Schwarzenegger";
        &action_hero[0..6]
    };
    println!("first name: {first_name}");

    let full_name = &action_hero[..];
    println!("full name: {full_name}");

    // Length of string slice
    // The length of a string slice refers to a count of its bytes, not its characters.
    let food = "🍕";
    println!("{}", food.len());

    // let pizza_slice = &food[0..3]; // byte index 3 is not a char boundary; it is inside '🍕' (bytes 0..4) of `🍕`
    // println!("{}", pizza_slice.len());

    let action_hero = String::from("Arnold Schwarzenegger");
    do_hero_stuff(&action_hero);

    let another_action_hero = "Sylvester Stallone";
    do_hero_stuff(another_action_hero);

    // Array slices
    let values = [4, 8, 15, 16, 23, 42];
    let array_slice = &values[..4];
    println!("{array_slice:?}");

    let regular_reference = &values;
    print_length(regular_reference);

    let slice_of_three = &values[..3];
    print_length(slice_of_three);

    // Mutable array slices
    // (note that mutable string slices does not take place in rust)
    let mut mut_array = [10, 15, 20, 25, 30];
    let mut_slice = &mut mut_array[2..4];
    println!("mut slice: {:?}", mut_slice);

    mut_slice[0] = 100;
    println!("mut slice: {:?}", mut_slice);
    println!("mut array: {:?}", mut_array);
}

// &String -> &str
fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day")
}

fn print_length(reference: &[i32]) {
    println!("{}", reference.len());
}
