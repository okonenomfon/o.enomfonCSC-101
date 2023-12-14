fn main() {

    let v = vec![101, 250, 330, 400];
    // vector v owns the object in the heap

    // only a single variable owns heap memory at any given time
    let v2 = v;

    println!("{:?}", v);
}
