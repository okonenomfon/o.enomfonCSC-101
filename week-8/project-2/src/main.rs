use std::io;

fn main() {
    let mut devs_input = String::new();

    println!("Enter the number of developers:");
    io::stdin()
        .read_line(&mut devs_input)
        .expect("Failed to read line");

    let num_devs: u32 = devs_input.trim().parse().expect("Failed to parse");

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
    let mut maximum_exp = 0;

    for &(ref name, exp) in &dev_data {
        if exp > maximum_exp {
            maximum_name = name.clone();
            maximum_exp = exp;
        }
    }

    println!(
        "Developer with the highest experience: {} with {} years",
        maximum_name, maximum_exp
    );

}
