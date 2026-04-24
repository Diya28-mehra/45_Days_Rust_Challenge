use std::io;
use std::collections::HashMap;
fn main() {

    //Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.

    let mut vec = Vec::new();
    vec.push(5);
    vec.push(6);
    vec.push(7);
    vec.push(8);
    vec.push(8);
    vec.push(9);
    println!("{:?}", vec);

    let len = vec.len();
    if len%2!=0{
        println!("The median of the vector is {}", vec[len/2]);
    }
    else{
        let median = (vec[len/2-1]+vec[len/2])/2;
        println!("The median of the vector is {}", median);
    }

    let mut map = HashMap::new();
    for i in 0..vec.len(){
        let count = map.entry(vec[i]).or_insert(0);
        *count+=1;
    }

    let mut mode = vec[0];
    let mut max_count = 0;
    for (key,value) in &map{
        if *value > max_count{
            max_count = *value;
            mode = *key;
        }
    }
    println!("The mode of the vector is {}", mode);

    //Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word 
    //and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end 
    //instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!


    let mut string = String::from("Hello World IceCream" );
    let mut result = String::new();

    for word in string.split_whitespace(){
        let mut chars = word.chars();
        if let Some(c)=chars.next(){
            if is_vowel(c){
                result.push_str(word);
                result.push_str("-hay");
            }
            else{
                let rest: String = chars.collect();
                result.push_str(&rest);
                result.push_str("-");
                result.push(c);
                result.push_str("ay ");
            }
        }
    }
    println!("The modified string is {}", result);



    //Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
    // for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then, let the user retrieve a list of all people in a department
    // or all people in the company by department, sorted alphabetically.For example, if the user types
    // “Add Sally to Engineering,” “Add Amir to Sales,” “Add Bob to Sales,” and “Add Alice to Engineering,” 
    //then the user should be able to type “Show Engineering” and see a list of all people in engineering, or “Show Sales” 
    //and see a list of all people in sales, or “Show All” and see a list of all people in the company by department, sorted alphabetically.


    let mut company = HashMap::new();
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input.to_lowercase()=="exit"{
            break;
        }
        else if input.to_lowercase().starts_with("add"){
            let parts = input[4..].split(" to ").collect::<Vec<&str>>();
            let employee = parts[0];
            let department = parts[1];
            company.entry(department.to_string()).or_insert(Vec::new()).push(employee.to_string());
        }
        else if input.to_lowercase().starts_with("show "){
            let department = input[5 ..].trim();
            if department.to_lowercase()=="all"{
                for (dept,employees) in &company{
                    println!("{}:{:?}",dept,employees);
                }
            }
            else if let Some(employees) = company.get(department){
                println!("{}:{:?}",department,employees);
            }
            else{
                println!("Department not found.");
            }
        }
        else{
           println!("Invalid command. Please try again.");
        }
    }
}

fn is_vowel(c:char)->bool{
    match c{
        'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U'=>true,
        _=>false,
    }
}
