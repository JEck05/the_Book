use std::{io::{self, Write}, cmp::Ordering, num::ParseIntError};
use rand::Rng;

fn main() {
    let rand_num: u32 = get_random_number();

    let mut count: u32 = 0;

    loop {

        println!("Guess the Number");

        print!("Your Guess:");

        let guess: u32 = match get_user_number(){
            Ok(num) => {count += 1; num},
            Err(_) => continue,
        };

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too Small!!"),
            Ordering::Equal => {println!("Correct!!"); break;},
            Ordering::Greater => println!("Too Big!!")
        }
    }
    print!("Number of guesses: {} \nThe number was {}", count, rand_num);
}
fn get_user_number() -> Result<u32, ParseIntError> {
    let mut guess = String::new();
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut guess)
        .unwrap();
    guess.trim().parse()
}
fn get_random_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}