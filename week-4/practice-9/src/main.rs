fn main(){
    let a = 10;
    let b = 20;

    println!("Value of A:{}",a );
    println!("Value of B:{}",b );
    
    let mut res = a>b;
    println!("A is greater than B: {}",res);

    res = a<b;
    println!("A is less than B: {}",res);

    res = a>=b;
    println!("A is greater than or equal to B: {}",res);

     res = a<=b;
    println!("A is less than or equal to B: {}",res);

     res = a==b;
    println!("A is equal to B: {}",res);

     res = a != b;
    println!("A is not equal to B: {}",res);

    
}