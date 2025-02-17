struct DentistAppointment {
    doctor: String,
}

impl DentistAppointment {
    // fn book<'a, 'b, 'c>(&'a self, check_in_time: &'b str, check_out_time: &'c str) -> &'a str {
    //     println!(
    //         "You are booked from {} to {} with doctor {}",
    //         check_in_time, check_out_time, self.doctor
    //     );

    //     &self.doctor
    // }

    fn book(&self, check_in_time: &str, check_out_time: &str) -> &str {
        println!(
            "You are booked from {} to {} with doctor {}",
            check_in_time, check_out_time, self.doctor
        );

        &self.doctor
    }
}

fn main() {
    let appointment = DentistAppointment {
        doctor: String::from("David"),
    };

    let result = appointment.book("03:00PM", "04:00PM");
    // drop(appointment) // error
    println!("{result:?}");
}
