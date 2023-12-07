use std::io::Write;

fn main() {

    let announce ="Week 9 - RUST FILE INPUT & OUTPUT\n";
    let dept = "DEPARTMENT OF SOFTWARE ENGINEERING";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust programing\n"
        .as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\nData written to file.");

}
