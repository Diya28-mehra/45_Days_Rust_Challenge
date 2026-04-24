pub struct NewsArticle{
    pub author: String,
    pub headline: String,
    pub content: String,
}

//Traits 
impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{} ,by {}",self.headline,self.author)
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summarize(&self)-> String{
        format!("{}: {}",self.username,self.content)
    }
}

pub trait Summary{
    fn summarize(&self)->String{
        String::from("Read More")
    }
}

//Traits as parameters
// pub fn modify(item: &impl Summary){
//     println!("Breaking News! {}",item.summarize());
// }

//Trait Bound
pub fn modify<T: Summary>(item:&T){
    println!("Breaking News! {}",item.summarize());
}

//when both items are of same type
//pub fn modify<T:Summary>(item1:&T,item2:&T){}

//Multiple Trait Bounds
//pub fn modify(item1: &(impl Summary+Display), item2: &impl Summary){}

//pub some_fn<T:Display+Clone,U:Clone+Debug>(t:&T,u:&U)->i32{}


//Returning Types that implement Traits
fn returns_summarizable()->impl Summary{
    Tweet{
        username:String::from("@jorhnup"),
        content:String::from("This is my first tweet How beautiful sky is just like you"),
        reply:false,
        retweet:false,
    }
}

fn main() {
    println!("Hello, world!");
    let na1 = NewsArticle{
        author:String::from("Diya"),
        headline:String::from("Lighten Up yourself"),
        content:String::from("Thsi is my autobiograpghy. My name is Diya which means whereever I go, it lightens up.")
    };
    println!("{}",na1.summarize());

    let tweet = Tweet{
        username:String::from("@jorhnup"),
        content:String::from("This is my first tweet How beautiful sky is just like you"),
        reply:false,
        retweet:false,
    };
    println!("{}",tweet.summarize());

    modify(&na1);

    println!("{}",returns_summarizable().summarize());
}
