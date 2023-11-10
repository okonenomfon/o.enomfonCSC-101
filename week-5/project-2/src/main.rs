//Rust program to determine incentive of employees

use std::io;

fn main() {

    let mut exp = String::new();
    let mut input2 = String::new();

    let a = 1_560_000;
    let b = 1_480_000;
    let c = 1_300_000;
    let d = 100_000;

     println!("Are you experienced: ");
        io::stdin()
        .read_line(&mut exp) 
        .expect("Not a valid string");
    let experienced:bool = exp.trim().parse().expect("Not valid");

    println!("Enter your age: ");
        io::stdin()
        .read_line(&mut input2) 
        .expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if experienced == true && age >= 40 {
        println!("Incentive is {}", a);
    }
    else if experienced == true && age >= 30 && age < 40 {
        println!("Incentive is {}", b);
    }
    else if experienced == true && age < 30 && age >= 18 {
        println!("Incentive is {}", c);
    }
    else if experienced == false && age >= 18 {
        println!("Incentive is {}", d);
    }
    else {
        println!("We don't employ less than legal age");
    }
    
}
