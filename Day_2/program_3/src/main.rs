fn main() {
    let mut s = String::from("hello My name is Diya Mehra");
    let word = first_word(&s);
    //s.clear(); //Will give an error because we are borrowing as mutable so first make s mutable and then clear it.
    //but immutable reference(word) is still in use, so we cannot clear it.
    println!("The first word is:{}",word);
    s.clear(); // now allowed
   // println!("The first word is:{}",word); cant print after clearing the string because word is still referencing the original string which is now empty.
}

fn first_word(s:&str)->&str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
