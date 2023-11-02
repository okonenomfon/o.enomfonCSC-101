fn main() {
    let n1 = "Electrical ".to_string();
    let n2 = "Engineering".to_string();
    let n3 = n1 + &n2; //n2 reference is passed

    //About Electrical Engineering
    println!("\nThe {} is informed by the aspiration to train Electrical Engineering professionals in the areas of design, building and maintenance of electrical control systems,", n3);
    

    let w1 = "Software ".to_string();
    let w2 = "Engineering".to_string();
    let w3 = w1 + &w2; //w2 reference is passed
    println!();
    println!("{} is aimed at developing competent, creative, innovative, enterpreneurial and ethically-minded persons, capable of creating value in the field of Software Engineering. ", w3);


}
