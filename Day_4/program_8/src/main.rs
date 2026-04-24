#[derive(Debug)]
struct Point<T>{
    x:T,
    y:T,
}

#[derive(Debug)]
struct Diff<T,U>{
    x:T,
    y:U,
}

impl<U> Point<U>{
    fn x(&self)->&U{
        &self.x
    }
}

impl Point<f64>{
    fn y(&self)->&f64{
        &self.y
    }
}

impl<T,U> Diff<T,U>{
    fn mixup<V,W> (self, other:Diff<V,W>)->Diff<T,W>{
        Diff{
            x:self.x,
            y:other.y,
        }
    }
}

fn main() {
    //Generics with Structs 
    let p1 = Point{x:5,y:0};
    println!("{}",p1.x());
    let p2 = Point{x:'a',y:'b'};
    // let p3 = Point{x:10,y:'a'}; //give error as both of T type both should be of same type.
    println!("{:?}",p2.x());

    let p3 = Point{x:10.0,y:12.30};
    println!("{:?}",p3.y());


    let d1 = Diff{x:10,y:'c'};
    let d2 = Diff{x:'c',y:110};
    println!("{:?}", d1.mixup(d2));

    enum Option<T>{
        Some(T),
        None,
    }

    enum Option1<T,E>{
        Ok(T),
        Err(E),
    }
}

