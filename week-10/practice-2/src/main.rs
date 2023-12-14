fn main() {

    let v = vec![10, 20, 30];
    // vector v owns the object in the heap

    // only a single variable owns heap memory at any given time
    let v2 = v;

    display(v2.clone());

    println!("In main {:?}", v2);
}

fn display(v:Vec<i32>){
    println!("Inside display {:?}", v);
}