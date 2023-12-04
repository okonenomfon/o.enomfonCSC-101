fn main() {

    let num = [1, 2, 3, 4, 5];
    println!("Initial array = {:?}", num);

    let slice1 = &num[1..3];
    println!("2nd and 3rd elements sliced = {:?}", slice1);

    let slice2 = &num[..3];
    println!("0 - 3 elements sliced = {:?}", slice2);

    let slice3 = &num[2..];
    println!("2 - 5 elements sliced = {:?}", slice3);

    let slice4 = &num[..];
    println!("0 - 5 elements sliced = {:?}", slice4)
}
