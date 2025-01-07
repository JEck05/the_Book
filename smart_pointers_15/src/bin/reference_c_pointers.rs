use std::rc::Rc;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List{
    Cons(i32,Rc<List>),
    Nil,
}
fn main(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Counts After Creating a {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("Counts After Creating b {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("Counts After Creating c {}", Rc::strong_count(&a));
    }
    println!("Counts After dropping c {}", Rc::strong_count(&a));
    
}