use traits::lodging::{utils, Accommodation, AirBnB, Description, Hotel};

fn main() {
    let mut hotel = Hotel::new("The luxe");
    println!("{}", hotel.summarize());
    hotel.book("Jigyo", 5);
    utils::book_for_one_night(&mut hotel, "Sunggon");
    println!("{:#?}", hotel);

    let mut airbnb = AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    airbnb.book("Youri", 3);
    utils::book_for_one_night(&mut airbnb, "Amanda");
    println!("{:#?}", airbnb);

    let mut hotel = utils::choose_best_place_to_stay();
    let mut airbnb = AirBnB::new("Peter");
    utils::mix_and_match(&mut hotel, &mut airbnb, "Jigyo");
    println!("{hotel:#?} {airbnb:#?}");

    let hotel1 = Hotel::new(String::from("The Luxe"));
    println!("{}", hotel1.summarize());
    let hotel2 = Hotel::new("The Golden Standard");
    println!("{}", hotel2.summarize());
    // let hotel3 = Hotel::new(vec!["The Sweet Escape"]);
    // println!("{}", hotel3.summarize()); // error

    let mut hotel = Hotel::new(String::from("The Luxe"));
    let mut airbnb = AirBnB::new("Peter");

    let stays: Vec<&dyn Description> = vec![&hotel, &airbnb];
    println!("{}", stays[0].get_description());
    println!("{}", stays[1].get_description());

    let mut stays: Vec<&mut dyn Accommodation> = vec![&mut hotel, &mut airbnb];
    stays[0].book("Jigyo", 2);
    stays[1].book("Youri", 3);
    println!("{:#?}", hotel);
    println!("{:#?}", airbnb);
}
