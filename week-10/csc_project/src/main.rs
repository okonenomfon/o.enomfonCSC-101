struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn calc_total(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let laptops = vec![
        Laptop { brand: String::from("HP"), price: 650_000 },
        Laptop { brand: String::from("IBM"), price: 755_000 },
        Laptop { brand: String::from("Toshiba"), price: 550_000 },
        Laptop { brand: String::from("Dell"), price: 850_000 },
    ];

    let mut total_cost = 0;

    for laptop in laptops.iter() {
        println!("Enter the quantity of {} laptops:", laptop.brand);

        let quantity: u32 = get_user_input();

        let brand_total_cost = laptop.calc_total(quantity);

        total_cost += brand_total_cost;
    }

    println!("Total cost for the selected laptops: {}", total_cost);
}

fn get_user_input() -> u32 {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Ok(value) = input.trim().parse() {
            return value;
        } else {
            println!("Please enter a valid number.");
        }
    }
}
