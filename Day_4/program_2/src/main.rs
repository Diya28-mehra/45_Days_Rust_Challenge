fn main() {

    let mut sb1 = String::from("foo");
    let sb2 = "bar";
    sb1.push_str(sb2);
    println!("sb2 is {sb2}");
    //Because push_str does NOT take ownership of s2.Instead, it borrows it.
    //Internally, push_str looks like: fn push_str(&mut self, string: &str)
    //So it takes:&mut self → mutable reference to sb1 &str → reference to string data

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let sc = s1.clone() + "-" + &s2 + "-" + &s3;
    println!("{}", sc);
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);

    //Rust strings don’t support indexing.
    //let h = s1[0]; //so it gives an error: cannot index into a `String` with `{integer}`

    let hello = String::from("Здравствуйте");
    println!("hello has {} bytes", hello.len());
    //hello has 24 bytes because each character in "Здравствуйте" takes 2 bytes in UTF-8 encoding.

    let answer = &hello[0..2];
    println!("answer is {answer}");
    //if answer is &hello[idx] it would give an error because Rust does not allow indexing into a String with an integer.
}
