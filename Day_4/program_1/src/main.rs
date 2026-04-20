fn main() {
    let mut v1 = Vec::new();
    v1 = vec![1,2,3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let vr = vec![1,3,4,5,2];
    let third = &vr[2];
    println!("The third element is {}", third);

    let thirrd :Option<&i32> = vr.get(2);
    match thirrd{
        Some(value) => println!("The third element is {}", value),
        None => println!("There is no third element."), 
    }

    //let does_not_exist = &v[100]; this will give to panic and crash the program
    let does_not_exist = v.get(100);
    match does_not_exist{
        Some(value) => println!("The 100th element is {}", value),
        None => println!("There is no 100th element."), 
    }


    let first = &v[0];
    v.push(6);
    //println!("The first element is: {first}"); //this will give to error since immutable reference is still in use 
    //when we try to modify the vector by pushing a new element. 
    //Rust's borrowing rules prevent us from modifying the vector while we have an immutable reference to one of its elements.


    for i in &v{
        println!("{}", i);
    }

    for i in &mut v{
        *i += 50;
        println!("{}", i);
    }
}
