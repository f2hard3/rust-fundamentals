use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let contents = read_to_string("story.txt")?;

    for line in contents.lines() {
        println!("{line}");
    }

    Ok(())
}
