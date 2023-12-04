fn main() {

    let mut colors = ["red", "green", "yellow", "white"];
    println!("Initial array = {:?}", colors);

    let sliced_colors = &mut colors[1..3];
    println!("1st sliced = {:?}", sliced_colors);

    sliced_colors[1] = "purple";
    println!("Changed slice = {:?}", sliced_colors);

}
