#[derive(Debug)]
struct TrainSystem<'a> {
    name: &'a str,
}

fn main() {
    let name = String::from("NJ Transit");
    let nj_transit = TrainSystem { name: &name };

    println!("{nj_transit:#?}");

    // let nj_transit = {
    //     let name = String::from("NJ Transit");
    //     TrainSystem { name: &name } // error
    // };

    // println!("{nj_transit:#?}");
}
