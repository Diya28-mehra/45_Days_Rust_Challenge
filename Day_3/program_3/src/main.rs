#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6,
}

struct IpAddress{
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    //Enums and Pattern Matching

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddress{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddress {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    route(home.kind);


    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    let x = 9;
    let y = Some(5);    
    let sum = x + y.unwrap_or(0);
    println!("The sum is: {}", sum);

}

fn route(ip_kind: IpAddrKind){
    println!("Routing to {:?}",ip_kind);
}