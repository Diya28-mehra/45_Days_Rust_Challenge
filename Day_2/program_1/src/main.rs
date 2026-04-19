fn main() {
    //Ownership 

    //Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.


    let mut s = String::from("hello");
    //s is the owner of the string "hello"
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);

    //Rust calls drop automatically at the closing curly bracket.

    let x = 5;
    let y = x;
    //In this case, x and y are both valid and equal to 5.
    //This is because integers are stored on the stack, and copying them is a simple.


    let s1 = String::from("Hey Diya");
    let s2 = s1;
    //In this case, s1 is no longer valid after the assignment to s2.
    //This is because the String type is stored on the heap, and copying it would be expensive.
    // Instead, Rust moves the ownership of the string from s1 to s2.

    //println!("{}", s1); // This will cause a compile-time error because s1 is no longer valid.
    println!("{}", s2); // This will print "Hey Diya" 


    //If we do want to deeply copy the heap data of the String, not just the stack data,
    // we can use a common method called clone

    let s3 = String::from("Hello");
    let s4 = s3.clone();
    //In this case, s3 and s4 are both valid and contain the same string
    println!("s3 = {}, s4 = {}", s3, s4); // This will print "s3 = Hello, s4 = Hello"


    let str = String::from("Makes Copy");
    takes_ownership(str);
    //println!("{}", str); // This will cause a compile-time error because str is no longer

    let x = 5;
    makes_copy(x);
    println!("{}", x); // This will print 5 because x is still valid after the function call 
    //because integers implement the Copy trait, which means that they are copied rather than moved when passed to a function.


    let word1 = gives_ownership(); // gives_ownership moves its return value into word1
    let word2 = String::from("Hello");
    let word3 = takes_and_gives_back(word2); // takes_and_gives_back moves its parameter into word2 and also returns it, moving it back into word2
    println!("word1: {}, word3: {}", word1, word3); // This will print "word1: Hello, word3: Hello"
}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn takes_and_gives_back(s: String) -> String { // takes_and_gives_back will move its return value into the function that calls it
    s // s is returned and moves out to the calling function
}

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("Hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}