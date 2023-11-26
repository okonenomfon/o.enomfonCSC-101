use std::io;

fn main() {
    println!("Enter the number of siblings:");
    let mut num_sibs = String::new();
    io::stdin().read_line(&mut num_sibs).expect("Failed to read line");
    let num_sibs: u32 = num_sibs.trim().parse().expect("Invalid input");

    let mut sibs_data: [String; 10] = Default::default();


    for s in 1..=num_sibs {
        println!("Enter details for Sibling {}: ", s);

        let mut sib_info = String::new();

        let mut name = String::new();
        println!("Enter first name:");
        io::stdin().read_line(&mut name).expect("Failed to read line");
        sib_info.push_str(&format!("Name: {}, ", name));

        let mut a = String::new();
        let age: u32;
        println!("Enter age:");
        io::stdin().read_line(&mut a).expect("Failed to read line");
        age = a.trim().parse().expect("Invalid input");
        sib_info.push_str(&format!("Age: {}, ", age));

        if age > 18 {
            println!("Is the sibling married? yes/no:");
            let mut m = String::new();
            io::stdin().read_line(&mut m).expect("Failed to read line");

            if m.trim() == "yes" {
                println!("How many offsprings?");
                let mut child = String::new();
                io::stdin().read_line(&mut child).expect("Failed to read line");
                sib_info.push_str(&format!("Marital Status: Married, Offspring: {}, ", child.trim()));

                println!("What city does the family live in?");
                let mut city = String::new();
                io::stdin().read_line(&mut city).expect("Failed to read line");
                sib_info.push_str(&format!("City: {}", city));
            } else {
                println!("Is the sibling a student or a worker?");
                let mut s_w = String::new();
                io::stdin().read_line(&mut s_w).expect("Failed to read line");

                if s_w.trim() == "student" {
                    println!("Enter university:");
                    let mut uni = String::new();
                    io::stdin().read_line(&mut uni).expect("Failed to read line");
                    sib_info.push_str(&format!("Marital Status: Single, Status: Student, University: {}, ", uni));

                    println!("Enter course:");
                    let mut course = String::new();
                    io::stdin().read_line(&mut course).expect("Failed to read line");
                    sib_info.push_str(&format!("Course: {}", course));
                } else {
                    sib_info.push_str("Marital Status: Single, Status: Worker");
                }
            }
        } else {
            println!("Do you have WAEC?");
            let mut waec = String::new();
            io::stdin().read_line(&mut waec).expect("Failed to read line");

            if waec.trim() == "yes" {
                println!("Enter secondary school attended:");
                let mut sec = String::new();
                io::stdin().read_line(&mut sec).expect("Failed to read line");
                sib_info.push_str(&format!("WAEC Status: Yes, Secondary School: {}", sec));
            } else {
                println!("Enter current class:");
                let mut class = String::new();
                io::stdin().read_line(&mut class).expect("Failed to read line");
                sib_info.push_str(&format!("WAEC Status: No, Current Class: {}", class));
            }
        }

        sibs_data[s as usize] = sib_info;
    }

    println!("Sibling Data:");
    for (s, sib) in sibs_data.iter().enumerate() {
        if !sib.is_empty() {
            println!("Sibling {}: {}", s, sib);
        }
    }
}
