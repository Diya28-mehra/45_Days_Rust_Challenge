mod front_part_of_restraurant{
    pub mod hosting{
        pub fn add_to_waitlist(){}
        fn seat_at_table(){}
    }

    mod serving{
        fn take_order(){}
        fn server_order(){}
        fn take_payment(){}
    }

}


pub fn eat_at_restraurant(){
    //Absolute path 
    crate::front_part_of_restraurant::hosting::add_to_waitlist();

    //Relative Path
    front_part_of_restraurant::hosting::add_to_waitlist();

    //To access the function and module, we must make them public using the pub keyword.
    //since by default, all functions and modules are private.

    