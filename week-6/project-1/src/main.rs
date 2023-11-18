use std::io;

fn main() {
    println!("
                        PAN-ATLANTIC UNIVERSITY STUDENTS VOTING SYSTEM");

    println!("
    Hello there!
    Fill up the following information.");

    for _ in 0..150{
        
        println!("
        Enter your name:");
        let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

        println!("
        Enter your Student E-mail:");
        let mut s_mail = String::new();
        io::stdin()
        .read_line(&mut s_mail)
        .expect("Failed to read input");

        println!("
        Enter your Department:");
        let mut department = String::new();
        io::stdin()
        .read_line(&mut department)
        .expect("Failed to read input");

        println!("
        Enter your Level:");
        let mut level = String::new();
        io::stdin()
        .read_line(&mut level)
        .expect("Failed to read input");
        let l:i32 = level.trim().parse().expect("Failed to input");

        println!("
        Are you a course rep?:");
        let mut post = String::new();
        io::stdin()
        .read_line(&mut post)
        .expect("Failed to read input");
        let class_rep:bool = post.trim().parse().expect("Failed to input");

        println!("
        Enter your Grade:");
        let mut grade = String::new();
        io::stdin()
        .read_line(&mut grade)
        .expect("Failed to read input");
        let g:f32 = grade.trim().parse().expect("Failed to input");
        
        println!("
        Enter your State:");
        let mut state = String::new();
        io::stdin()
        .read_line(&mut state)
        .expect("Failed to read input");


        if class_rep == true 
        && l > 100
        && g > 4.0 {
            println!("
            Name: {}
            Student E-mail: {}
            Department: {}
            State of Origin: {}", name, s_mail, department, state);
            println!("
            You can continue with the voting process");
        }
        else {
            println!("
            Name: {}
            Student E-mail: {}
            Department: {}
            State of Origin: {}", name, s_mail, department, state);
            println!("
            You are not eligible to vote");
        }

    } 
}
