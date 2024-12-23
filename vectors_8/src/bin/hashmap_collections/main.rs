use std::collections::{HashMap};

fn main(){
    let mut map:HashMap<String, i32> = HashMap::from([
        (String::from("Blue"), 2),
        (String::from("Red"), 3)]
    );
    // Uses closure to ensure "50 + 20" is only evaluated when the Key is not in the map
    let _ = map.entry(String::from("glue")).or_insert_with( || { println!("hello"); 50 + 20} );

    for(key, value) in &map{
        println!("{key}, {value}");
    }
    let _ = HashMap::from(map);

    word_frequency(String::from("hello hello hello world world hi"));
}
fn word_frequency(input_str: String){
    let mut map = HashMap::new();

    for word in input_str.split_whitespace(){
        // entry returns a mutable reference to the value that corresponds
        // to the given key
        let count_value = map.entry(word).or_insert(0);
        *count_value += 1;
        assert_eq!(&String::from(word), word);
    }
    print!("{map:?}");
}