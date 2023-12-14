fn main() {

    let v = vec![15, 25, 35, 45, 55];
    print_vector(v.clone());
    println!("{}", v[0]);
}

fn print_vector(x:Vec<i32>){
    println!("Inside display {:?}", x);
}