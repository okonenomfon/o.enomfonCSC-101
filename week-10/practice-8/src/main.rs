struct Employee{
    ceo: String,
    company: String,
    age: u32
}

fn main() {

    let e1 = Employee{
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };
    let e2 = Employee{
        company:String::from("Google Inc."),
        ceo:String::from("Sundai Pichai"),
        age:51
    };

    display(e1);
    display(e2);
}

fn display(e:Employee){
    
    println!("Name is {}, Company is {}, Age is {} ", e.ceo, e. company, e.age);

}
