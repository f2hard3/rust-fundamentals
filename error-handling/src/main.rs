use std::fs;
use std::io::{self, stdin};
use std::process;

fn main() {
    let file_result = read_file();
    match file_result {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprintln!("There was an error: {error:#?}")
        }
    }

    let mut animals = vec!["Giraffe", "Monkey", "Zebra"];
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));

    match write_to_file() {
        Ok(path) => println!("Successfully wrote to {path}"),
        Err(error) => {
            eprintln!("There was an error: {error}");
            process::exit(1);
        }
    }
}

fn read_file() -> Result<String, io::Error> {
    println!("Please enter the name of the file you'd like to read:");

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    fs::read_to_string(input.trim())

    // let mut file_content = String::new();
    // File::open(input.trim())?.read_to_string(&mut file_content)?;

    // Ok(file_content)
}

fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last_element = input.pop()?;
    Some(last_element.len())
    // Some(input.pop()?.len())
}

fn write_to_file() -> io::Result<String> {
    let input = stdin();
    println!("What file would you like to write to?");
    let mut path = String::new();
    input.read_line(&mut path)?;

    println!("What would you like to write to the file?");
    let mut contents = String::new();
    input.read_line(&mut contents)?;

    fs::write(path.trim(), contents.trim())?;

    Ok(path)
}
