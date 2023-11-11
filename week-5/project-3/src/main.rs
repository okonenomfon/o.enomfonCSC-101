use std::io;

fn main() {
    println!("Welcome to E's Cafe!");

    let mut serve = String::new();

    println!("How do we serve you?");
    io::stdin()
        .read_line(&mut serve)
        .expect("Failed to read line");

    let place_order: &str = serve.trim();

    println!("Here is our Menu: (N/B: If you make an order of #10,000 and above, you will be given a discount of 5%)");

    let items = ["Pounded yam and Edikanikon soup", "Fried rice and Chicken", "Eba and Egusi soup", "Amala and Ewedu soup", "White rice and Stew"];
    let prices = [3500, 3000, 2500, 2000, 2500];

    for food in 0..items.len() {
        println!("{}: #{}", items[food], prices[food]);
    }

    // Type of food
    let mut choice = String::new();

    println!("Enter the type of food:");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    // Quantity of food
    let mut quantity = String::new();

    println!("Enter the quantity you need:");
    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read line");
    let qty: u32 = quantity.trim().parse().expect("not a valid quantity");

    // Index of the user's choice
    if let Some(p) = items.iter().position(|&item| item.trim() == choice.trim()) {
        
        let price = prices[p];

        // Calculate total price
        let total_price = price * qty;

        // Applying a 5% discount
        let discount = if total_price > 10000 {
            total_price - (total_price * 5 / 100)
        } else {
            total_price
        };

        println!("You ordered {} {} Total Price: #{}", quantity, choice, discount);

        // Another order
        let mut another = String::new();

        println!("Would you like to place another order? (Yes/No)");
        io::stdin()
            .read_line(&mut another)
            .expect("Failed to read line");

        if another.trim() == "Yes" {
            println!("Please continue with your additional order.");
        } 
        else {
            println!("Thank you for your order!");
        }
    } else {
        println!("{} is not on the menu.", choice);
    }
}
