use std::fmt::Display;
// This is an example of lifetimes in structs
// these are required when a struct holds references
struct ImportantExcerpt<'a>{
    part: &'a str
}
impl<'a> ImportantExcerpt<'a>{
    fn level(&self) -> i32{
        3
    }
    // This shows how explicit lifetime declaration is not necessary
    // because all lifetimes are accounted for
    fn announce_and_return_part(&self, announcement: &str) -> &str{
        println!("{announcement}");
        self.part
    }
}
fn main(){
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{} {} {} {}",
             i.level(),
             i.part,
             i.announce_and_return_part("Hello"),
             longest_with_an_announcement(i.part, i.part, "Hello"));
}
// This shows the combine usage of generics and lifetimes in one function
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T, ) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
