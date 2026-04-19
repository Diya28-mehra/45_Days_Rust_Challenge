#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin, 
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::California);
    let value = find_value(coin);
    println!("The value of the coin is: {} cents", value);

    let one = Some(1);
    let two = Some(2);
    let ans = plus_one(one);
    let ans2 = plus_one(two);
    let ans3 = plus_one(None);
    println!("The answer is: {:?}", ans);
    println!("The answer is: {:?}", ans2);
    println!("The answer is: {:?}", ans3);


    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    
}

fn find_value(coin: Coin)->u64{
    match coin{
        Coin::Penny=>{
            println!("Lucky penny!");
            1
        },
        Coin::Nickel=>5,
        Coin::Dime=>10,     
        Coin::Quarter(state)=>{
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x:Option<i32>)->Option<i32>{
    match x{
        None => None,
        Some(i)=>Some(i+1),
    }
}