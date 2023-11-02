fn main() {
    let fullname = "Enomfon Mfon Okon";
    let dept = "Software Engineering";
    let uni = "Pan-Atlantic University";

    let mut school = "School of Science ".to_string();
    //push string
    school.push_str("and Technology");

    println!("My name is {}", fullname);

    //check length
    println!("The length of my full name is {}",fullname.len());
    println!("I am a student of {} Department", dept);
    println!("{}",school);
    println!("{}",uni);

}
