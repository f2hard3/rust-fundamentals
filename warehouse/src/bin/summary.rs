use fake::{Fake, Faker};
use warehouse::{Item, ProductCategory, FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER};

/// Create a summary of our warehouse
fn main() {
    println!(
        "Our manager are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );

    let favorite_category = ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}");

    let tall_ladder = Item::new(String::from("Ladder-o-matic 2000"), favorite_category, 100);
    println!("{tall_ladder:?}");

    let fake_item: Item = Faker.fake();
    println!("{fake_item:?}");

    let random_category = Faker.fake::<ProductCategory>();
    println!("{random_category:?}");
}
