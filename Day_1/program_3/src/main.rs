use rand::Rng;
fn main() {
    //Data Types in Rust

    //Scalar Types = Integer, Floating-point, Boolean, Character, Numeric 
    //A scalar type represents a single value 
    let x :i32 = 10;
    let y : f64 = 2.39;
    let sum = x+y as i32;
    println!("The sum of x and y is: {}",sum);

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z :char = 'Z';

    //Compound Types = Tuple, Array, Slice, Struct, Enum

    let tup : (i32,f64,char) = (500,3.45,'c');
    let (a,b,c) = tup;
    println!("The value of a is: {}",a);
    println!("The value of b is: {}",b);        
    println!("The value of c is: {}",c);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let ab = [3; 5];


    let number = rand::thread_rng().gen_range(1..=10);
    if number <5{
        println!("The number is less than 5");
    }
    else{
        println!("The number is greater than or equal to 5");
    }
}
