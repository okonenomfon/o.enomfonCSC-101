fn main() {

    let v: Vec<i32> = vec![20, 40, 60, 80];
    // vector v owns the object in the heap

    // only a single variable owns heap memory at any given time
    let _v2 = v.clone();
    let v2_return = display(v);

    println!("In main {:?}", v2_return);
}

fn display(v: Vec<i32>) -> Vec<i32>{
    println!("Inside display {:?}", v);
    return v;
}