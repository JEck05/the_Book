

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    // this is invalid due to result being only valid as long as string1 is valid(unchanged)
    // so because we use result later on it is invalid
    // string1 = String::from("s");

    // same for this, because we specify the lifetime in longest() to be
    // both as long as string1 and string2 this is invalid
    // string2 = "s";

    println!("{string1}");

    println!("{string2}");

    println!("The longest string is {result}");
    
}
fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str{

    if string1.len() > string2.len() {
         string1
    }else {
         string2
    }
}
