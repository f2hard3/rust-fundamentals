fn main() {
    let cities = vec![
        String::from("New York"),
        String::from("Tokyo"),
        String::from("Seoul"),
    ];

    // let favorite_cities = &cities[..2];
    // println!("{favorite_cities:?}"); // non-lexical scope
    // let places = cities;

    let two_cities = {
        let cities_reference = &cities;
        select_first_two_element(cities_reference)
    };
    // drop(cities); // error
    println!("{two_cities:?}");
    {
        let coffees = [String::from("Latte"), String::from("Mocha")];
        let two_coffees = select_first_two_element(&coffees);
        println!("{two_coffees:?}");
    }
}

// fn create() -> &i32 {
//     let age = 100;
//     &age
// }

// fn create_slice(items: Vec<i32>) -> &[i32] {
//     &items[..2]
// }

// fn create_number_reference(number: i32) -> &i32 {
//     &number
// }

fn select_first_two_element<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2]
}
