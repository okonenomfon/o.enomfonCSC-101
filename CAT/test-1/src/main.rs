use std::io;

fn main() {
    println!("
                        Otunene Patient Information Management System");

    for _ in 0..100 {
        let d1 = "Alzheimer - #1_200_000 - Akpabom";
        let d2 = "Arrhythmia - #550_000 - Ngbauji";
        let d3 = "Chronic Kidney Disease - #1_500_000 - Atabrikang";
        let d4 = "Diabetes - #800_000 - Okorobilom";
        let d5 = "Arthritis - #450_000 - Emeremen";

        println!("
        We treat the following:");
        println!(
            "{}\n{}\n{}\n{}\n{}",
            d1, d2, d3, d4, d5
        );

        println!("\nPlease enter medical diagnosis.");
        let mut med = String::new();
        io::stdin()
            .read_line(&mut med)
            .expect("Failed to read input");

        println!("\nPlease enter your name.");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");
        println!("Name: {}", name);

        println!("\nPlease enter your age.");
        let mut age = String::new();
        io::stdin()
            .read_line(&mut age)
            .expect("Failed to read input");
        println!("Age: {}", age);

        println!("\nPlease enter your E-mail address.");
        let mut email = String::new();
        io::stdin()
            .read_line(&mut email)
            .expect("Failed to read input");
        println!("Email address: {}", email);

        println!("\nPlease enter your phone number.");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read input");
        println!("Phone Number: {}", number);

        println!("\nPlease enter your number of siblings.");
        let mut sib = String::new();
        io::stdin()
            .read_line(&mut sib)
            .expect("Failed to read input");
        println!("Sibling(s): {}", sib);

        println!("\nPlease enter the amount of children you have.");
        let mut child = String::new();
        io::stdin()
            .read_line(&mut child)
            .expect("Failed to read input");
        println!("Number of Child(ren): {}", child);

        println!("\nPlease enter your village of residence.");
        let mut vil = String::new();
        io::stdin()
            .read_line(&mut vil)
            .expect("Failed to read input");
        println!("Village of Residence: {}", vil);

        let a1 = 1_200_000;
        let a2 = 550_000;
        let a3 = 1_500_000;
        let a4 = 800_000;
        let a5 = 450_000;

        let mut discount = 0;
        let mut price = 0;

        if med.trim() == "Alzheimer"
            && age.trim().parse::<i32>().unwrap() > 50
            && child.trim().parse::<i32>().unwrap() > 4
            && vil.trim() == "Akpabom"
        {
            discount = a1 - (a1 * 20 / 100);
        } else if med.trim() == "Arrhythmia"
            && age.trim().parse::<i32>().unwrap() == 30
            && sib.trim().parse::<i32>().unwrap() > 4
            && vil.trim() == "Ngbauji"
        {
            discount = a2 - (a2 * 5 / 100);
        } else if med.trim() == "Chronic Kidney Disease"
            && age.trim().parse::<i32>().unwrap() > 40
            && sib.trim().parse::<i32>().unwrap() > 3
            && child.trim().parse::<i32>().unwrap() > 3
            && vil.trim() == "Atabrikang"
        {
            discount = a3 - (a3 * 15 / 100);
        } else if med.trim() == "Diabetes"
            && age.trim().parse::<i32>().unwrap() > 28
            && age.trim().parse::<i32>().unwrap() < 45
            && child.trim().parse::<i32>().unwrap() >= 2
            && child.trim().parse::<i32>().unwrap() <= 4
            && vil.trim() == "Okorobilom"
        {
            discount = a4 - (a4 * 10 / 100);
        } else if med.trim() == "Arthritis"
            && age.trim().parse::<i32>().unwrap() > 58
            && sib.trim().parse::<i32>().unwrap() > 5
            && child.trim().parse::<i32>().unwrap() > 5
            && vil.trim() == "Emeremen"
        {
            discount = a5 - (a5 * 10 / 100);
        }

        if discount > 0 {
            println!("
            Your condition: {} - Discounted Price: #{}", med, discount);
        } 
        else {
            price = match med.trim() {
                "Alzheimer" => a1,
                "Arrhythmia" => a2,
                "Chronic Kidney Disease" => a3,
                "Diabetes" => a4,
                "Arthritis" => a5,
                _ => 0,
            };

            println!("
            Your condition: {} - Price: #{}", med, price);
        }
    }
}
