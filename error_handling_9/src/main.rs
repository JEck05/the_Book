fn main() {
    println!("Hello, world!");
    

    let v = vec![1,2,3];
    let v2:Vec<i32> = Vec::from([1,2,3]);

    let _ = v2[2];
    //Accessing out of range will panic
    let _ = v[99];
    
    //Invoking a closure (|| {//code to execute})() 
    panic!("{}", ( || { println!("Hi"); "Help me".to_string() })());
}
