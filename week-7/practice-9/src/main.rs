fn main() {

    let arr1:[i32;4] = [10,20,30,40];
    println!("Array is {:?}", arr1);
    println!("Array size is: {}", arr1.len());

    for val in arr1.iter() {
        println!("Value is: {}", val);
    }
}
