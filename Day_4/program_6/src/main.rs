use std::io;
use std::io::Read;
use std::fs::File;
use std::fs;
use std::error::Error;

fn main() ->Result<(),Box<dyn Error>>{
    
    //Error Propagation - when a function has an implementation that calls something 
    //that could possible fail


    // "?" returns an option so we can't use it directly under main 

    let f = File::open("hello.txt")?;
    Ok(())
}


fn read_username_from_file()->Result<String,io::Error>{
    let f = File::open("baby.txt");

    let mut f = match f{
        Ok(file)=>file,
        Err(e)=>return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_)=>Ok(s),
        Err(e)=>Err(e),
    }
}

fn read_userbame_from_file_usingshortcut()->Result<String,io::Error>{
    let mut f = File::open("hello.text")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file_moreshortcut()->Result<String,io::Error>{
    let mut s = String::new();
    File::open("hello.text")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_oneline() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
