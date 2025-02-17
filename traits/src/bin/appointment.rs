#[derive(Debug, Clone)]
struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}

// impl Clone for Appointment {
//     fn clone(&self) -> Self {
//         Self {
//             doctor: self.doctor.clone(),
//             start_time: self.start_time.clone(),
//             end_time: self.end_time.clone(),
//         }
//     }
// }

#[derive(Debug, Clone)]
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Duration {
    fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

impl Copy for Duration {} // Copy relies on Clone(super trait).

fn main() {
    let morning_appointment = Appointment::new("Dr. Jigyo", "09:00AM", "10:00AM");
    let replacement_appointment = morning_appointment.clone();
    println!(
        "{} is seeing the patient from {} to {}",
        replacement_appointment.doctor,
        replacement_appointment.start_time,
        replacement_appointment.end_time
    );
    println!("{:?}", morning_appointment);

    let one_hour = Duration::new(1, 0, 0);
    let another_hour = one_hour;
    println!("{:?}", one_hour);
}
