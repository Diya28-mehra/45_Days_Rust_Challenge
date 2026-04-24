use std::fs::File;
use std::io::ErrorKind;
fn main() {
    //Error Handling in Rust
    // panic!("crash and run");
    //a();
    //can run with RUST_BACKTRACE=1 cargo run

    enum Result<T,E>{
        Ok(T),
        Err(E),
    }



    let f = File::open("hii.txt");
    let f = match f{
        Ok(f)=>f,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound => match File::create("hii.txt"){
                Ok(fc)=>fc,
                Err(e)=>panic!("Problem in creating the file {:?}",e),
            }
            other_error=>panic!("Problem in opening the file {:?}",other_error),
        }
    };



    let f = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Problem creating the file: {:?}",error);
            })
        }else{
            panic!("Problem opening the file: {:?}",error);
        }
    });

    let ft = File::open("watch.txt").expect("failed to open file");


}

fn a(){
    b();
}

fn b(){
    c(22);
}

fn c(num: i32){
    if num==22 {
        panic!("Don't pass in 22!");
    }
}