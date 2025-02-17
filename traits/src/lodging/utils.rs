// use super::lodging::{Accommodation, Description, Hotel};
use crate::lodging::{Accommodation, Description, Hotel};
use std::fmt::Debug;

// fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
//     println!("{}", entity.get_description());
//     entity.book(guest, 1);
// }

pub fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    println!("{}", entity.get_description());
    entity.book(guest, 1);
}

// fn mix_and_match(
//     first: &mut (impl Accommodation + Description),
//     second: &mut impl Accommodation,
//     guest: &str,
// ) {
//     first.book(guest, 1);
//     first.get_description();
//     second.book(guest, 1);
// } // first and second params could be different

pub fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    second.book(guest, 1);
} // T should be the same type, so U is needed

pub fn choose_best_place_to_stay() -> impl Accommodation + Description + Debug {
    Hotel::new("The Luxe")
}
