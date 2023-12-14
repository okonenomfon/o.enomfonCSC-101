struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self)->u32 {
        self.width * self.height
    }
}

fn main() {

    let small = Rectangle {
        width:10,
        height:20
    };

    println!("Width is {} \n and Height is {} \n therefore, Area of Rectangle is {} ", small.width, small.height, small.area());

}
