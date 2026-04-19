mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){
            println!("add to waitlist");
        }
    }
}
//Absolute path
//use crate::front_of_house::hosting;

//Relative path
use self::front_of_house::hosting;
pub fn eat_at_restraurant(){
    hosting::add_to_waitlist();
}
