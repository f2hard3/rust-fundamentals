use std::cmp::Ordering;

struct Job {
    salary: u32,
    commute_time: u32,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.salary.partial_cmp(&other.salary)

        // if self.salary == other.salary {
        //     Some(Ordering::Equal)
        // } else if self.salary < other.salary {
        //     Some(Ordering::Less)
        // } else if self.salary > other.salary {
        //     Some(Ordering::Greater)
        // } else {
        //     None
        // }
    }
}

fn main() {
    let long_commute_job = Job {
        salary: 100_000,
        commute_time: 2,
    };

    let short_commute_job = Job {
        salary: 75_000,
        commute_time: 1,
    };

    println!("{}", long_commute_job > short_commute_job);
    println!("{}", long_commute_job >= short_commute_job);
    println!("{}", long_commute_job <= short_commute_job);
    println!("{}", long_commute_job < short_commute_job);
    println!("{}", long_commute_job == short_commute_job);
    println!("{}", long_commute_job.gt(&short_commute_job));
}
