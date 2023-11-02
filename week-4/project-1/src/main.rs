//rust program to calculate the speed of a car in km

fn main() {
    let s1:f32 = 80.0;     //the unit here is in mile
    let s2:f32 = 120.0;     //the unit here is in mile

    let t1:f32 = 2.0;     //the unit here is in hours
    let t2:f32 = 4.0;     //the unit here is in hours'
    
    //converting mile to kilometer
    let k1:f32 = 1.60934;  //1mile = 1.60934km

    let km1:f32;

    km1 = s1 * k1;
    println!("km1 = {}",km1);

    let km2:f32;

    km2 = s2 * k1;
    println!("km2 = {}",km2);

    //solving for the speed
    let speed_1:f32;

    speed_1 = km1 / t1;
    println!("Speed for the 1st value = {} kmph",speed_1);

    let speed_2:f32;

    speed_2 = km2 / t2;
    println!("Speed for the 2nd value = {} kmph",speed_2);

}
