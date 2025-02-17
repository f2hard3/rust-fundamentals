#[allow(unused_variables)]
fn main() {
    // Copy trait
    let time = 2025;
    let year = time; // copy(different ownership[time, year])
    println!("The time is {time}. It is the year {year}");

    // String type
    let food = "pasta"; // binary executable(not a stack nor a heap) that the compiler produce, reference(pointer)
    let text = String::new(); // heap
    let candy = String::from("KitKat"); // KitKat: binary executable, candy: heap

    // push_str method on a String type
    let mut name = String::from("Sunggon");
    println!("{name}");

    name.push_str(" Park");
    println!("{name}");

    // Moves and Ownership
    let person = String::from("Sunggon"); // String does not implement Copy trait.
    println!("My name is {person}");

    let developer = person; // ownership moved from person to developer, person goes out of scope.

    // println!("My name is {person}") // error

    // Drop function
    let person = String::from("Sunggon");
    drop(person);
    // println!("{person}"); // error
    // let genius = person; // error

    // Clone method
    let person = String::from("Sunggon");
    let genius = person.clone(); // Clone trait
    println!("{person}");

    // References and Borrowing
    let my_stack_value = 2;
    let my_stack_reference = &my_stack_value; // &: borrow

    let my_heap_value = String::from("Toyota"); // remains the owner of the String
    let my_heap_reference = &my_heap_value; // owner of the reference

    // reference: guaranteed to point to a valid value(referent).
    // pointer: does not have the guarantee.
    // reference must never outlive the existence of the value in rust.

    // Dereference Operator
    // dereference: to access the data at the memory address that the reference points to.
    let my_stack_value = 2;
    let my_stack_reference: &i32 = &my_stack_value;
    println!("{}", *my_stack_reference);
    // println!("{}", *my_stack_value); // error

    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;
    println!("{}", my_heap_reference); // Display trait
    println!("{}", *my_heap_reference);

    // The copy trait with references
    let ice_cream = "Cookies and Cream";
    let dessert = ice_cream;
    println!("{}", ice_cream);
    println!("{}", dessert);

    // Ownership and function parameters
    let apples = 6;
    print_my_stack_value(6);
    println!("{apples} is still valid");

    let oranges = String::from("Oranges");
    print_my_heap_value(oranges);
    // println!("{oranges} is still valid"); // error

    // Mutable parameters
    let burger = String::from("Burger");
    add_fries(burger);

    // Return values I
    let cake = bak_cakes();
    println!("I now have a {cake} cake");

    // Return values II
    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);

    // Multiple immutable references
    let car = String::from("Red");
    let ref1 = &car;
    let ref2 = &car;
    println!("{ref1} and {ref2} and {}", &car);

    // Mutable reference restrictions
    let mut car = String::from("Red");
    let ref1 = &mut car;
    ref1.push_str(" and Silver");
    let ref2 = &car;
    // println!("{ref1} and {ref2}"); // error
    // ref1.push_str("string"); // error
    println!("{ref2}");

    // Ownership with immutable and mutable references
    let mut coffee = String::from("Mocha");
    let a = &mut coffee;
    let b = a; // same as let b = &coffee

    // println!("{a} {b}"); // error

    // Dangling reference
    // let reference = create_city();

    // Ownership with Arrays and Tuples
    let registrations = [true, false, true];
    let first = registrations[0];
    println!("{first} and {registrations:?}");

    let languages = [String::from("Rust"), String::from("Javascript")];
    let first = &languages[0]; // the most idiomatic Rust approach
    println!("{first} and {languages:?}");

    // let first = languages[0].clone();

    let registrations = (true, false, true);
    let first = registrations.0;
    println!("{first} and {registrations:?}");

    let languages = (String::from("Rust"), String::from("Javascript"));
    let first = &languages.0;
    println!("{first} and {languages:?}");
} // drop(person) <- for heap allocated memory only

fn print_my_stack_value(value: i32) {
    println!("Your value is {value}");
}

// value: movement or ownership
fn print_my_heap_value(value: String) {
    println!("Your value is {value}");
}

fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{meal}")
}

fn bak_cakes() -> String {
    String::from("Chocolate Mousse")
}

fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}");
}

// Dangling reference
// fn create_city() -> &String {
//     let city = String::from("Tokyo");
//     &city
// }
