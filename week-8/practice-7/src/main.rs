fn main() {

    let tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Tuple contents = {:?}", tuple);

    let no_tuple = ("Rust", "fun", 100);
    println!("Tuple contents = {:?}", no_tuple);

    println!("Value at Index 0 = {}", tuple.0);

    println!("Value at Index 1 = {}", tuple.1);

    println!("Value at Index 2 = {}", tuple.2);
}
