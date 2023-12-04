use std::io;

fn main() {
    let mut devs_input = String::new();

    println!("Enter the number of developers:");
    io::stdin()
    .read_line(&mut devs_input)
    .expect("Failed to read line");

    let num_devs: u32 = match devs_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for the number of developers. Defaulting to 0.");
            0
        }
    };

    let mut dev_data: Vec<(String, i32)> = Vec::new();

    for i in 0..num_devs {
        let mut dev_name = String::new();
        let mut dev_exp = String::new();

        println!("Enter your name {}: ", i + 1);
        io::stdin()
        .read_line(&mut dev_name)
        .expect("Failed to read line");

        println!("Enter years of experience {}: ", i + 1);
        io::stdin()
        .read_line(&mut dev_exp)
        .expect("Failed to read line");

        let exp: i32 = dev_exp.trim().parse().expect("Failed to read line");

        dev_data.push((dev_name.trim().to_string(), exp));
    }

    let mut maximum_name = "No developers".to_string();
    let mut maximum_exp:i32 = 0;

    for developer in &dev_data {
        let (name, exp) = developer;
        if exp > &maximum_exp {
            maximum_name = name.to_string();
            maximum_exp = *exp;
        }
    }

    println!(
        "Developer with the highest experience: {} with {} years",
        maximum_name, maximum_exp
    );
}
