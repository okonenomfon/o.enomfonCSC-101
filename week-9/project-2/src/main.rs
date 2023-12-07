use std::fs::File;
use std::io::Write;

fn main() {

    let student_names = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Bianca Edemoh"];
    let matric_numbers = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let departments = vec!["Accounting", "Economics", "Computer Science", "Electrical Engineering", "Mechanical Engineering"];
    let levels = vec!["300", "100", "200", "200", "100"];

    let file_name = "student_details.txt";

    let mut file = File::create(file_name).expect("Error: Unable to create or open the file.");

    file.write_all(b"           STUDENT INFORMATION MANAGEMENT SYSTEM\n").expect("Error writing to file");

    file.write_all(format!("{:<20} {:<20} {:<30} {:<10}\n", "Name", "Matric No.", "Department", "Level")
        .as_bytes())
        .expect("Error writing to file");

    for n in 0..student_names.len() {
        file.write_all(
            format!("{:<20} {:<20} {:<30} {:<10}\n", student_names[n], matric_numbers[n], departments[n], levels[n])
            .as_bytes()
        )
        .expect("Error writing to file");
    }

    println!("Student details have been written to {}", file_name);
}
