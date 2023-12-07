use std::fs::File;
use std::io::Write;

fn main() {

    let lagers = vec![
        String::from("33 Export"),
        String::from("Desperado"),
        String::from("Goldberg"),
        String::from("Gulder"),
        String::from("Heineken"),
        String::from("Star"),
    ];

    let stouts = vec![
        String::from("Legend"),
        String::from("Turbo King"),
        String::from("Williams"),
    ];

    let non_alcoholic = vec![
        String::from("Maltina"),
        String::from("Amstel Malta"),
        String::from("Malta Gold"),
        String::from("Fayrouz"),
    ];

    let drink_categories = vec![lagers, stouts, non_alcoholic];

    let file_name = "drink.txt";

    let mut file = File::create(file_name)
        .expect("Error creating or writing to the file");

    let category_names = ["Lager", "Stout", "Non_Alcoholic"];

    for (index, category) in drink_categories.iter().enumerate() {

        let category_name = category_names.get(index).expect("Unknown category");

        let category_header = format!("{}:\n", category_name);
        if let Err(error) = file.write_all(category_header.as_bytes()) {
            println!("Error writing to the file: {}", error);
            return;
        }

        for drink_variety in category {
            let variety_line = format!("= {}\n", drink_variety);
            if let Err(error) = file.write_all(variety_line.as_bytes()) {
                println!("Error writing to the file: {}", error);
                return;
            }
        }
    }

    println!("Drink categories saved to drink.txt");
}
