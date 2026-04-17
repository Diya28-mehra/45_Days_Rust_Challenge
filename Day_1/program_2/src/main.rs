fn main() {
    // This code will not compile because we are trying to assign a new value to an immutable variable `x`.
    // let x = 10;
    // x=5;

    let mut x=10;
    println!("The value of x is: {}",x);
    x=5;
    println!("The value of x is: {}",x);

    //shadowing allows us to declare a new variable with the same name as a previous variable. 
    //The new variable shadows the previous variable, and we can use the same name for different variables in different scopes.

    let y=10;
    let y = y+1;
    {
        let y = y*2;
        println!("The value of y in the inner scope is: {}",y);
    }
    println!("The value of y in the outer scope is: {}",y);
}
