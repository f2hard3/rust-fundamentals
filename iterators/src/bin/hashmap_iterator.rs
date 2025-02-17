use std::collections::HashMap;

fn main() {
    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);
    println!("{:?}", todos);

    for (todo, completion_status) in todos {
        println!("Task: {todo}, Complete: {completion_status}");
    }
    // println!("{:?}", todos); // error

    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);
    println!("{:?}", todos);

    for (todo, completion_status) in &todos {
        println!("Task: {todo}, Complete: {completion_status}");
    }
    println!("{:?}", todos);

    let mut todos = HashMap::new();
    todos.insert("Pick up groceries", false);
    todos.insert("Study Rust", true);
    todos.insert("Sleep", false);
    println!("{:?}", todos);

    for (_, completion_status) in &mut todos {
        *completion_status = true;
    }
    println!("{:?}", todos);
}
