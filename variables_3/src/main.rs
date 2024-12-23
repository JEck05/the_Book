#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut s = String::from("hello world");

    let x = first_word(&s);

    println!("{}", x);

    s.clear();

    println!("{}", s);

    let x: User = build_user(String::from("email"), String::from("name"));

    println!("Active:{} Email:{} Username: {} Login Count:{}", x.active, x.email, x.username, x.sign_in_count);

    let user2: User = User{
        email: String::from("emal2"),
        ..x
    };

    println!("{user2:#?}");
    dbg!(&user2);
}
// Returns the first word in a string(Space is the delimiter)
fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
   &s[..]
}
fn build_user(email:String, username:String) -> User{
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

