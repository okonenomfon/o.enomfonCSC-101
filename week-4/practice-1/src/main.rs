fn main(){
    let name = "Okon, Enomfon Mfon";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {} ", name);
    println!("University: {}, \nAddress: {}", uni,addr);
    

    let dept:&'static str = "Software Engineering";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool: {}", dept,school);
    
}