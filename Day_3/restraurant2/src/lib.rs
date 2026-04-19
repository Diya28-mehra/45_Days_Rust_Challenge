mod back_of_house{
    pub struct Breakfast{
        pub toast:String,
        pub seasonal_fruit:String,
    }
    impl Breakfast{
        pub fn summer(toast:String)->Breakfast{
            Breakfast{
                toast,
                seasonal_fruit:String::from("peaches"),
            }
        }
    }

    pub enum Appetizer{
        Soup,
        Salad,
    }
}

pub fn eat_at_restraurant(){
    let mut mean = back_of_house::Breakfast::summer(String::from("Avacado Toast"));
    //mean.toast = String::from("Rye"); //error toast is private by default

    println!("I'd like {} toast please", mean.toast); //can't access toast since it's private

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;


}