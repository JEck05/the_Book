use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor{
    Red,
    Blue
}
struct Inventory{
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor{
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        }else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory{
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!("The user with the pref {user_pref1:?}, gets {giveaway1:?}");

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!("The user with the pref {user_pref2:?}, gets {giveaway2:?}");


    // This example shows that closures types(that arent explicit declared)
    // are inferred by how it is being used in this case the closure variable is String
    let example_closure = |x| x;

    let _s = example_closure(String::from("hello"));
    // let n = example_closure(5);  // This does not compile because it is type String
    closures_as_variables();
    closures_taking_ownership();
}
fn closures_as_variables() -> (){
    // How closures can capture an immutable reference
    // and how it can be called later on as an immutable reference
    // Notice that you can have an immutable reference after
    // the closure is declared(as long as the closure in immutable aswell)
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");

    only_borrows();

    println!("After calling closure: {list:?}");
    {
        // here is an example of a closure capturing a mutable reference
        // notice after the closure captures the mutable variable
        // you cannot use it immutable after or borrow it again without
        // the closure being out of scope
        let mut borrows_mutably = || list.push(10);
        //this would cause an error because it borrows it as immutable
        // print!("Before {list:?}");
        borrows_mutably();
        println!("After calling closure: {list:?}");
    }

}
fn closures_taking_ownership(){
    let list = vec![1,2,3];

    println!("Before Defining Closure {list:?}");
    // Even though list is an immutable reference, because we are using threads
    // we need to "move" ownership to the new thread because it is possible the new thread
    // could finish before main, or vice versa therefore making list a dangling reference
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

