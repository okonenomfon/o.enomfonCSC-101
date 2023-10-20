fn main() {
    //amount
    let t:f64 = 450_000.0;
    let m:f64 = 1_500_000.0;
    let h:f64 = 750_000.0;
    let d:f64 = 2_850_000.0;
    let a:f64 = 250_000.0;

    //quantity
    let qt:f64 = 2.0;
    let qm:f64 = 1.0;
    let qh:f64 = 3.0;
    let qd:f64 = 3.0;
    let qa:f64 = 1.0;

    //sum
    let s = (t * qt) + (m * qm) + (h * qh) + (d * qd) + (a * qa);
    println!("Sum is {}", s);
    let tq = qt + qm + qh + qd + qa;
    println!("Total quantity is {}", tq);
    let mean = s/tq;
    println!("Mean(Average) is {}", mean);

}