mod inventory;
mod orders;

// use std::{
//     fmt,
//     io::{self, stdin, stdout},
// };
// use std::collections::*;

use fake::{Fake, Faker};

use inventory::{Item, ProductCategory, MANAGER as INVENTORY_MANAGER};
use orders::MANAGER as ORDERS_MANAGER;

/// Primary entry point into our warehouse program
fn main() {
    println!(
        "Our manager are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER,
        ORDERS_MANAGER,
        crate::inventory::FLOOR_SPACE
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
