//Generic stack Progam
#[derive(Debug)]
struct Stack<T>{
    item: Vec<T>,
}

impl<T> Stack<T>{
    fn new()->Self{
        Stack{
            item: Vec::new(),
        }
    }

    fn push(&mut self,value:T){
        self.item.push(value)
    }

    fn pop(&mut self)->Option<T>{
        self.item.pop()
    }

    fn peek(&mut self)->Option<&T> {
        self.item.last()
    }

    fn is_empty(&mut self)->bool{
        self.item.is_empty()
    }

    fn size(&self)->usize{
        self.item.len()
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(12);
    stack.push(4);

    println!("Stack: {:?}", stack);
    println!("Top: {:?}", stack.peek());

    stack.pop();
    println!("After pop: {:?}", stack);

    // String stack
    let mut stack2 = Stack::new();
    stack2.push("Hello");
    stack2.push("Rust");

    println!("String Stack: {:?}", stack2);
 
}
