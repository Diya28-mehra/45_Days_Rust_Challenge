#[derive(Debug)]
struct rectangle{
    length: u32,
    width: u32,
}

impl rectangle{
    fn area(&self)->u32{
        self.length*self.width
    }
    fn can_hold(&self,other: &rectangle)->bool{
        self.length>other.length && self.width>other.width
    }
    // this is an associated function because it does not take self as a parameter
    //often used for constructors that will return a new instance of the struct. 
    fn square(size:u32)-> Self{
        Self{
            length: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = rectangle{
        length: 10,
        width: 5,
    };
    println!("The area of rectangle is:{}",rect1.area());

    let rect2 = rectangle {
        length: 10,
        width: 40,
    };
    let rect3 = rectangle {
        length: 60,
        width: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}
