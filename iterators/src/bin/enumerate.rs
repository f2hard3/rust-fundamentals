fn main() {
    let applicants = vec!["Rob", "Bob", "Cob", "Alex", "Piers", "John", "Dan"];
    let winners: Vec<&str> = applicants
        .into_iter()
        .enumerate()
        .filter_map(|(index, applicant)| {
            if index % 3 == 0 {
                Some(applicant)
            } else {
                None
            }
        })
        .collect();
    println!("winner: {winners:?}");
}
