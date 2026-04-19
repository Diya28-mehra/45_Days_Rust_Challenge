fn main() {
    //References 
    //To save the ownership of the data, we can use references.
    //References are immutable by default, but we can make them mutable by using the 'mut' keyword.
    let s1 = String::from("Hello");
    let len = calculate_length(&s1); //We pass a reference to the function
    println!("The length of '{}' is {}.", s1, len);

    //Mutable references
    let mut s = String::from("hello");
    change(&mut s);


    //At any given time, you can have either one mutable reference or any number of immutable references.
    ///References must always be valid.

    let mut sb = String::from("hello");

    let r1 = &sb; // no problem
    let r2 = &sb; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut sb; // no problem
    println!("{r3}");


}

fn calculate_length(s:&String)->usize{
    s.len()
}

fn change(s:&mut String) {
    s.push_str(", world!");
}