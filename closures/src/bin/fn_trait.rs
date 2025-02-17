fn execute_thrice<F>(procedure: F)
where
    F: Fn(),
{
    procedure();
    procedure();
    procedure();
}

fn execute_thrice_mut<F>(mut procedure: F)
where
    F: FnMut(),
{
    procedure();
    procedure();
    procedure();
}

fn bake_cake() {
    println!("Hello chocolate");
}

fn main() {
    // FnOnce
    // let people = vec!["Boris"];
    // let closure = || {
    //     let employees = people;
    // }; // error

    // Fn
    let closure = || println!("I am sunggon");
    execute_thrice(closure);

    // FnMut
    let mut people = vec!["Boris"];
    let closure = || {
        people.push("Jigyo");
    };
    execute_thrice_mut(closure);
    println!("{people:?}");

    // passing a function to Fn trait parameter
    execute_thrice(bake_cake);
    let option: Option<Vec<String>> = None;
    let collection = option.unwrap_or_else(Vec::new);
    println!("{collection:?}");
}
