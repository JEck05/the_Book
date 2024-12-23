use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let file_name = "hesllo.txt";
    let greeting_file_result = File::open(file_name);

    let _greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create_new(file_name){
                Ok(new_file) => new_file,
                Err(err) => panic!("unable to create file {err:?}"),
            },
            other_error=> {
                panic!("Problem opening file {other_error:?}");
            }
        },
    };

    let _greeting_file_2 = File::open(file_name).unwrap();
    let _ = read_username_from_file().unwrap();
}


fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file_result = File::open("hesllo.txt")?;
    let mut username = String::new();
    username_file_result.read_to_string(&mut username)?;
    //A simplified way of doing the above by chaining method calls with "?"
    // File::open("hesllo.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
fn last_char_of_first_line(text: &str) -> Option<char>{
    text.lines().next()?.chars().last()
}

