fn value(n:Option<&char>) {

    println!("Element of vector {:?}", n);

}

fn main() {

    let v = vec!['R', 'U', 'S', 'T', 'A', 'C', 'I', 'A', 'N'];

    let mut input1 = String::new();

    println!("Enter an index value between (0 - 8)");
    std::io::stdin()
    .read_line(&mut input1)
    .expect("Failed to read input");
    let index:usize = input1.trim().parse().expect("Invalid input");

    let ch:Option<&char> = v.get(index);
    value(ch);

}
