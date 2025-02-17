use std::io::stdin;

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    // fn unlock(self, procedure: impl FnOnce() -> String) -> Option<String>
    // fn unlock<F: FnOnce() -> String>(self, procedure: F) -> Option<String>
    fn unlock<F>(self, procedure: F) -> Option<String>
    where
        F: FnOnce() -> String,
    {
        let user_password = procedure();
        if user_password == self.password {
            Some(self.treasure)
        } else {
            None
        }
    }
}

fn main() {
    let vault = Vault {
        password: String::from("topSecret"),
        treasure: String::from("Gold"),
    };

    // FnOnce
    // let mut user_input = String::new();
    // println!("Please provide as password to crack the vault");
    // match stdin().read_line(&mut user_input) {
    //     Ok(usize) => println!("input length: {usize}"),
    //     Err(error) => println!("{error}"),
    // }
    // user_input = user_input.trim().to_string();
    // let hack = || user_input;

    // Fn
    let hack = || {
        let mut user_input = String::new();
        println!("Please provide as password to crack the vault");

        match stdin().read_line(&mut user_input) {
            Ok(usize) => println!("input length: {usize}"),
            Err(error) => println!("{error}"),
        }

        user_input.trim().to_string()
    };

    match vault.unlock(hack) {
        Some(treasure) => println!("{treasure}"),
        None => println!("No treasure"),
    }
}
