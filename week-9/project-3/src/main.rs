use std::fs::File;
use std::io::Write;

fn main() {

    let ministers_names = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministries = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let geo_zone = vec!["South West", "North East", "South South", "North Central", "South East"];

    let file_name = "convict.txt";

    let mut file = File::create(file_name).expect("Error: Unable to create or open the file.");

    file.write_all(format!("{:<50}\n", "CONVICTED MINISTERS\n")
        .as_bytes())
        .expect("Error writing to file");

    file.write_all(format!("{:<50} {:<50} {:<30}\n", "NAME", "MINISTRY", "GEOPOLITICAL ZONE")
        .as_bytes())
        .expect("Error writing to file");

    for n in 0..ministers_names.len() {
        file.write_all(
            format!("{:<50} {:<50} {:<30}\n", ministers_names[n], ministries[n], geo_zone[n])
            .as_bytes()
        )
        .expect("Error writing to file");
    }

    println!("Minister's details written to {}", file_name);
}
