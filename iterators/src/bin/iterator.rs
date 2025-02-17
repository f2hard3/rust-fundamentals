fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.into_iter();
    // println!("{:?}", my_vector); // error
    println!("{:?}", my_iterator);

    // for loop became the mutable owner of the iterator
    for number in my_iterator {
        println!("{}", number);
    }
    // println!("{:?}", my_iterator); // error

    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.iter();

    for number in my_iterator {
        println!("{}", number);
    }
    // println!("{:?}", my_iterator); // error

    for number in &my_vector {
        println!("{}", number);
    }
    println!("{:?}", my_vector);

    let cities = vec![
        String::from("Seoul"),
        String::from("Dallas"),
        String::from("Tokyo"),
        String::from("Osaka"),
    ];

    for city in &cities {
        println!("{}", city);
    }
    println!("{:?}", cities);

    let mut flavors = [
        String::from("Strawberry"),
        String::from("Chocolate"),
        String::from("Vanilla"),
    ];

    // let iterator = flavors.iter_mut();
    for flavor in &mut flavors {
        flavor.push_str(" Ice Cream");
    }
    println!("{:?}", flavors);

    let mut school_grades = [85, 90, 72, 92];
    for grade in &mut school_grades {
        *grade -= 2;
    }
    println!("{:?}", school_grades);
}
