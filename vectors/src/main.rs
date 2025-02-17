#[allow(unused_variables)]
fn main() {
    // array: fixed length
    let movies = ["Ghost", "Honey", "Space"];

    // vector: a flexible array that grows and shrinks in size (heap)
    let pastas: Vec<&str> = Vec::new();
    let pastas = Vec::<&str>::new();
    println!("{pastas:?}");

    let pastas: Vec<&str> = vec!["Rigatoni", "Angel hair", "Cream"];
    println!("{pastas:?}");

    let pizza_diameters: Vec<i32> = Vec::new();
    let pizza_diameters = Vec::<i32>::new();
    println!("{pizza_diameters:?}");

    let mut pizza_diameters = vec![8, 10, 12, 14];
    pizza_diameters.push(16);
    pizza_diameters.push(18);
    pizza_diameters.insert(0, 4);
    pizza_diameters.insert(2, 9);
    println!("{pizza_diameters:?}");

    let last_pizza_diameter = pizza_diameters.pop();
    println!("{:?}", last_pizza_diameter.unwrap());
    println!("{pizza_diameters:?}");

    let third_diameter_from_start = pizza_diameters.remove(2);
    println!("{third_diameter_from_start:?}");
    // pizza_diameters.remove(50); //error

    // Reading vector elements
    let pizza_diameters = vec![8, 10, 12, 14];

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];
    // println!("{:?}", pepperoni) // error
    // let value = pizza_toppings[2]; // error
    let reference = &pizza_toppings[2];
    println!("{:?}", reference);
    println!("{pizza_toppings:?}");

    let pizza_slice = &pizza_toppings[1..];
    println!("{pizza_slice:?}");

    let option = pizza_toppings.get(0);
    match option {
        Some(topping) => println!("{topping}"),
        None => println!("No value at that index position"),
    }
    println!("{pizza_toppings:?}");

    let mut delicious_topping = pizza_toppings;
    let topping_reference = &delicious_topping[1];
    println!("{topping_reference:?}");

    // delicious_topping.push(String::from("Olives")); //error
    // println!("{topping_reference:?}");

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let mut pizza_toppings = vec![pepperoni, mushroom, sausage];
    pizza_toppings[1] = String::from("Olives");
    println!("{pizza_toppings:?}");

    let target_topping = &mut pizza_toppings[2];
    // let another_target_topping = &pizza_toppings[2]; //error
    target_topping.push_str(" and Meatballs");
    let another_target_topping = &pizza_toppings[2];
    println!("{pizza_toppings:?}");

    // Vector capacity
    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!(
        "Length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );
    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");
    println!(
        "Length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    seasons.push("Summer");
    println!(
        "Length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );
}
