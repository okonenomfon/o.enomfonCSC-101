struct Employee{
    name: String,
    company: String,
    age: u32
}

fn main() {

    let e1 = Employee{
        company:String::from("Enrst & Young"),
        name:String::from("Edidiong Jessica"),
        age:25
    };
    println!("Name = {} \n", e1.name);
    println!("Company = {} \n", e1.company);
    println!("Age = {} ", e1.age);
}
