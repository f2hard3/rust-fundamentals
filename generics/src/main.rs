#[derive(Debug)]
struct DeliSandwich {}

#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}
impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}
impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}
impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

enum CheeseSteak<T> {
    Plain,
    Topping(T),
}

fn main() {
    println!("{}", identity::<i32>(5));
    println!("{}", identity::<i8>(5));
    println!("{}", identity::<u32>(5));
    println!("{}", identity(13.14));
    println!("{}", identity("hello"));
    println!("{}", identity(String::from("hello")));
    println!("{}", identity::<bool>(true));
    println!("{:?}", identity::<DeliSandwich>(DeliSandwich {}));

    println!("{:?}", make_tuple(3, "hello"));
    println!("{:?}", make_tuple(3, 13));
    println!("{:?}", make_tuple(true, 3.14));
    println!("{:?}", make_tuple(true, false));

    let gold_chest = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };
    println!("{:?}", gold_chest);
    println!("{:?}", gold_chest.capital_captain());

    let mut silver_chest = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: String::from("        Silver      "),
    };
    silver_chest.clean_treasure();
    println!("{:?}", silver_chest);
    println!("{:?}", silver_chest.capital_captain());

    let special_chest = TreasureChest {
        captain: String::from("Bootyplunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{:?}", special_chest);
    println!("{:?}", special_chest.amount_of_treasure());
    println!("{:?}", special_chest.capital_captain());

    let mushroom = CheeseSteak::Topping("mushroom");
    let onions = CheeseSteak::Topping("onions".to_string());
    let topping = "bacon".to_string();
    let bacon = CheeseSteak::Topping(&topping);

    let mut plain: CheeseSteak<String> = CheeseSteak::Plain;
    plain = CheeseSteak::Topping(String::from("sausage"));
}

fn identity<T>(value: T) -> T {
    value
}

fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}
