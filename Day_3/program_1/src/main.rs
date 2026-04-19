struct User{
    name: String,
    age: u32,
    email: String,
    active: bool,
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let mut user1 = User{
        name: String::from("Diya Mehra"),
        age:20,
        email: String::from("itsdiyamehra@gmail.com"),
        active:true,
    };
    user1.email = String::from("letsconnectdm@gmail.com"); //whole instance must be mutable to change any field of the struct.
    println!("User1's name is: {}",user1.name);

    let user2 = User{
        name: String::from("John Doe"),
        ..user1
    };

    println!("User2's name is: {}",user2.name);
    //println!("User1's email is: {}",user1.email); //this will give an error because we have moved the ownership of email field from user1 to user2, so we cannot access it through user1 anymore.


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
 

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:#?}");
    dbg!(&rect1);
}
