use std::collections::{HashMap, HashSet};

fn main() {
    // HashMap
    // let menu = HashMap::<String, f64>::new();
    let mut menu: HashMap<String, f64> = HashMap::new();
    menu.insert(String::from("Steak"), 29.99);
    menu.insert(String::from("Tuna"), 29.99);
    menu.insert(String::from("Burger"), 14.99);
    println!("{menu:?}");

    // let mut country_capitals: HashMap<&str, &str> = HashMap::new();
    let mut country_capitals = HashMap::new();
    country_capitals.insert("France", "Paris");
    country_capitals.insert("Germany", "Berlin");
    println!("{country_capitals:?}");

    let data = [("Bobby", 7), ("Grant", 4), ("Ben", 6)];
    let mut years_at_company = HashMap::from(data);
    println!("{:?}", years_at_company);

    let ben = years_at_company.remove("Ben");
    println!("{:?}", ben);
    println!("{:?}", ben.unwrap());
    println!("{:?}", years_at_company);
    let ben = years_at_company.remove("Ben");
    println!("{:?}", ben);

    let mut coffee_pairings = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(drink, milk);
    println!("{}", coffee_pairings.len());
    println!("{:?}", coffee_pairings);
    // println!("key:{} value:{}", drink, milk); // error

    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    println!("{:?}", coffee_pairings);
    println!("key:{} value:{}", drink, milk);
    coffee_pairings.insert("Flat White", "Almond Milk");
    println!("{:?}", coffee_pairings);

    let value = coffee_pairings["Flat White"];
    println!("{value}");
    println!("{:?}", coffee_pairings);
    let value = coffee_pairings
        .get("Cappuccino")
        .copied()
        .unwrap_or("Unknown Milk");
    println!("{:?}", value);

    coffee_pairings.insert("Latte", "Pistachio Milk");
    println!("{:?}", coffee_pairings);

    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    coffee_pairings.insert("Latte", "Almond Milk");
    coffee_pairings.insert("Flat White", "Oat Milk");

    coffee_pairings.entry("Latte").or_insert("Pistachio Milk");
    println!("{:?}", coffee_pairings);
    coffee_pairings
        .entry("Cappuccino")
        .or_insert("Pistachio Milk");
    println!("{:?}", coffee_pairings);

    // HashSet
    let mut concert_queue: HashSet<&str> = HashSet::new();
    concert_queue.insert("Sunggon");
    concert_queue.insert("Jigyo");
    println!("{:?}", concert_queue);
    println!("{}", concert_queue.len());
    concert_queue.insert("Jigyo");
    println!("{:?}", concert_queue);
    println!("{:?}", concert_queue.contains("Sunggon"));
    println!("{:?}", concert_queue.contains("Fred"));
    println!("{}", concert_queue.remove("Sunggon"));
    println!("{:?}", concert_queue);
    println!("{:?}", concert_queue.get("Sunggon"));
    println!("{:?}", concert_queue.get("Jigyo"));

    let mut concert_queue: HashSet<&str> = HashSet::new(); // Sunggon, Jigyo
    let mut movie_queue = HashSet::new(); // Jigyo, Phil

    concert_queue.insert("Sunggon");
    concert_queue.insert("Jigyo");

    movie_queue.insert("Jigyo");
    movie_queue.insert("Phil");

    println!("{:?}", concert_queue.union(&movie_queue));
    println!("{:?}", movie_queue.union(&concert_queue));

    println!("{:?}", concert_queue.difference(&movie_queue));
    println!("{:?}", movie_queue.difference(&concert_queue));

    println!("{:?}", concert_queue.symmetric_difference(&movie_queue));
    println!("{:?}", movie_queue.symmetric_difference(&concert_queue));

    println!("{:?}", concert_queue.is_disjoint(&movie_queue));
    println!("{:?}", movie_queue.is_disjoint(&concert_queue));

    let mut attendees = HashSet::new();
    attendees.insert("Jigyo");

    println!("{:?}", concert_queue.is_subset(&attendees));
    println!("{:?}", attendees.is_subset(&concert_queue));
    println!("{:?}", attendees.is_subset(&attendees));

    println!("{:?}", concert_queue.is_superset(&attendees));
    println!("{:?}", attendees.is_superset(&concert_queue));
    println!("{:?}", attendees.is_superset(&attendees));
}
