use std::io;

fn main() {

    println!("RESEARCHERS PUBLICATION INCENTIVE SYSTEM (RPIS)");

    for _ in 0..500 {

        println!("
        Enter name:");
        let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

        println!("
        Number of paper(s):");
        let mut paper = String::new();
        io::stdin()
        .read_line(&mut paper)
        .expect("Failed to read input");
        let p:i32 = paper.trim().parse().expect("Failed to input");

        let a1:i32 = 500_000;
        let a2:i32 = 800_000;
        let a3:i32 = 1_000_000;
        let a4:i32 = 100_000;

        if p >= 3 && p <= 5 {
            println!("
            Based on the number of papers published, your incentive is #{}", a1);
        }

        else if p > 5 && p <= 10 {
            println!("
            Based on the number of papers published, your incentive is #{}", a2);
        }

        else if p > 10 {
            println!("
            Based on the number of paper published, your incentive is #{}", a3);
        }

        else if p < 3 && p >= 1 {
            println!("
            Based on the number of papers published, your incentive is #{}", a4);
        }

        else {
            println!("
            Try to work harder.");
        }

    }

}
