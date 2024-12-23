use std::io::{self, Write};

fn main() {
    let v= vec![1,2,3];

    loop{
        print!("What index to you want to check? ");

        let user_index:usize= get_user_index();

        let number: Option<&i32> = v.get(user_index);
        
        match number {
            Some(val_at_index) => { println!("The value at index[{user_index}] is {val_at_index}") },
            None => { println!("There is no value at {user_index}");}
        }
    }
}
/// Returns an unsigned int from the user 
/// 
/// If input **cannot** be parsed to [`usize`] type, returns [`usize::MAX`](usize::MAX) and prints the [`ParseIntError`](core::num::ParseIntError) 
/// # Examples
///You can query a user for a specific index of a `Vector`:
/// ``` 
///let user_index:usize= get_user_index();
/// let val_at_index: Option<&i32> = vec.get(user_index);
///match val_at_index {
///   Some(val_at_index) => { println!("The value at index[{user_index}] is {val_at_index}") },
///   None => { println!("There is no value at {user_index}");}
///}
/// ```
/// 
fn get_user_index() -> usize {
    let mut user_index = String::new();
    io::stdout()
        .flush()
        .ok();
    io::stdin()
        .read_line(&mut user_index)
        .ok();
    let user_index = user_index.trim();
    user_index
        .parse::<usize>()
        .unwrap_or_else(|int_parse_err| {
                eprintln!("Error:{} \"{}\" Default value is set to {}(Max unsigned int size)", int_parse_err, user_index, usize::MAX);
                usize::MAX
            }
        )
}
