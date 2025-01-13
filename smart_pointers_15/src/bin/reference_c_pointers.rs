use std::cell::{RefCell};
use std::rc::Rc;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List{
    Cons(i32,Rc<List>),
    Nil,
}
enum BoxList{
    Cons(i32, Box<BoxList>),
    Nil
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
    reference_list();
}
#[derive(Debug)]
enum RefList{
    Cons(Rc<RefCell<i32>>, Rc<RefList>),
    Nil
}
fn reference_list(){
    let value = Rc::new(RefCell::new(10));

    let list_a = Rc::new(RefList::Cons(Rc::clone(&value), Rc::new(RefList::Nil)));

    let list_b: RefList = RefList::Cons(Rc::new(RefCell::new(8)), Rc::clone(&list_a));

    println!("value before = {value:?}");
    println!("a before = {list_a:?}");
    println!("b before = {list_b:?}");

    *value.borrow_mut() += 1;

    println!("value after = {value:?}");
    println!("a after = {list_a:?}");
    println!("b after = {list_b:?}");

}