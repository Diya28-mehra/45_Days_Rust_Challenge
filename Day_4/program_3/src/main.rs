use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // let score = scores.get(&team_name);
    // println!("score is {score:?}");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score is {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //we will not be able to use field_name and field_value as ownership has been passed to map collections

    map.entry(String::from("Favorite color")).or_insert("Red".to_string());
    map.entry(String::from("Favourite Icecream")).or_insert("Vanilla".to_string());

    println!("{:?}", map.get("Favorite color"));
    println!("{:?}", map.get("Favourite Icecream"));

    let text = "hello world wonderful world";
    let mut mp = HashMap::new();

    for word in text.split_whitespace(){
        let count = mp.entry(word).or_insert(0);
        *count+=1;
    }

    println!("{:?}", mp);
}
