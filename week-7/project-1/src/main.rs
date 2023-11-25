use std::io;

fn a_trap(a: f64, b: f64, c: f64) -> f64 {
    a / 2.0 * (b + c)
}

fn a_rhm(d: f64, e: f64) -> f64 {
    (1.0 / 2.0) * d * e
}

fn a_parl(f: f64, g: f64) -> f64 {
    f * g
}

fn a_cube(h: f64) -> f64 {
    6.0 * h.powf(2.0)
}

fn v_cyl(r: f64, h: f64) -> f64 {
    (22.0 / 7.0) * r.powf(2.0) * h
}

fn main() {
    println!("Select an equation:");
    println!("1. Area of trapezium");
    println!("2. Area of rhombus");
    println!("3. Area of parallelogram");
    println!("4. Area of cube");
    println!("5. Volume of cylinder");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    println!("Enter the first number:");
    let mut n1 = String::new();
    io::stdin().read_line(&mut n1).expect("Failed to read line");
    let n1: f64 = match n1.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    println!("Enter the second number:");
    let mut n2 = String::new();
    io::stdin().read_line(&mut n2).expect("Failed to read line");
    let n2: f64 = match n2.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    println!("Enter the third number:");
    let mut n3 = String::new();
    io::stdin().read_line(&mut n3).expect("Failed to read line");
    let n3: f64 = match n3.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    if choice == 1 {
        println!("Result: {}", a_trap(n1, n2, n3));
    } else if choice == 2 {
        println!("Result: {}", a_rhm(n1, n2));
    } else if choice == 3 {
        println!("Result: {}", a_parl(n1, n2));
    } else if choice == 4 {
        println!("Result: {}", a_cube(n1));
    } else if choice == 5 {
        println!("Result: {}", v_cyl(n1, n2));
    } else {
        println!("Invalid choice. Please select a valid equation.");
    }
}
