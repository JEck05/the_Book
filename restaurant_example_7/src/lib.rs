mod front_of_house;
pub fn eat_at_restaurant(){
    crate::front_of_house::hosting::add_to_waitlist();
    println!("hi");
    front_of_house::hosting::add_to_waitlist();
}