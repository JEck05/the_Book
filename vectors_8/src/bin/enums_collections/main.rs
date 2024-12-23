
fn main() {

    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(4),
        SpreadsheetCell::Float(2.2),
        SpreadsheetCell::Text(String::from("Hello")),
    ];
    for i in row {
        let _ = match i {
            SpreadsheetCell::Int(i) => i.to_string(),
            SpreadsheetCell::Float(f) => f.to_string(),
            SpreadsheetCell::Text(t) => t.to_string(),
        };
    }
    let s1 = String::from("Hi");
    let s2 = String::from(" world");
    let s3 = s1 + &s2;

    let s = format!("{s2}{s3}");
    println!("{s}");
}

