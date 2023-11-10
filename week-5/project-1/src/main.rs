//Rust program to calculate quadratic equation

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Value for a: ");
        io::stdin()
        .read_line(&mut input1)
        .expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Value for b: ");
        io::stdin()
        .read_line(&mut input2)
        .expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Value for c: ");
        io::stdin()
        .read_line(&mut input3)
        .expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");
    
    let d:f32 = (b * b) - (4.0 * a * c);

    if d > 0.0 {
        println!("There are two distinct roots");
    }
    else if d == 0.0 {
        println!("There is only one real root");
    }
    else if d < 0.0 {
        println!("There is no real root")
    }
    
}
