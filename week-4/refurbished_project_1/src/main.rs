//Rust program to calc. speed

use std::io;

fn main() {

    let mut distance = String::new();
    let mut time = String::new();


    println!("Enter the distance covered: ");
        io::stdin()
        .read_line(&mut distance)
        .expect("Not a valid string");
    let a:f32 = distance.trim().parse().expect("Not a valid number");

    println!("Enter the time taken to travel: ");
        io::stdin()
        .read_line(&mut time)
        .expect("Not a valid string");
    let b:f32 = time.trim().parse().expect("Not a valid number");


    let km:f32 = a * 1.60934;   //converting to kilometer
    let speed:f32 = km / b;
    
    println!("The speed of the journey in kilometer = {}", speed);
    
}
