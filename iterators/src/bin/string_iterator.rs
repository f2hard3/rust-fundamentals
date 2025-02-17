fn main() {
    let seafood = "Oyster🦪";

    for byte in seafood.bytes() {
        print!("{byte}/");
    }
    println!("{seafood}");

    for char in seafood.chars() {
        print!("{char}/");
    }
    println!("{seafood}");

    println!("{}", seafood.bytes().len());
    println!("{}", seafood.chars().count());
}
