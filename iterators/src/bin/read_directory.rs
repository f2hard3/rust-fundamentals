// use std::{fs::read_dir, io::Result, process::exit};
use std::{
    fs::{metadata, read_dir, read_to_string},
    io::Result,
};

fn main() -> Result<()> {
    // let directory = read_dir("./").unwrap_or_else(|error| {
    //     eprintln!("Could not read directory: {error}");
    //     exit(1);
    // });

    // for entry_result in directory {
    //     match entry_result {
    //         Ok(entry) => println!("{:?}", entry.path()),
    //         Err(error) => eprintln!("Could not read entry: {error}"),
    //     }
    // }

    for entry_result in read_dir("./")? {
        // match entry_result {
        //     Ok(entry) => println!("{:?}", entry.path()),
        //     Err(error) => eprintln!("Could not read entry: {error}"),
        // }

        // if let Ok(entry) = entry_result {
        //     println!("{:?}", entry.path())
        // }

        let entry = entry_result?;
        let path = entry.path();
        let metadata = metadata(&path)?;
        if metadata.is_file() {
            println!("{path:?}\n----------");
            let contents = read_to_string(&path)?;
            println!("{contents}")
        }
    }

    Ok(())
}
