
struct CustomSmartPointer{
    data:String
}
fn main(){
    // Variables are dropped in reverse order
    // so d then c
    let c = CustomSmartPointer{
        data: String::from("Hello")
    };
    let _d = CustomSmartPointer{
        data: String::from("Hello World")
    };
   
    println!("Custom Pointers Created");
    // We can also call the drop function directly
    // but only through std::mem::drop
    // This is because at the end of a function
    // It will still call c.drop() which would be a double free error
    // so we pass it to drop which takes ownership
    drop(c);
    println!("Custom Pointer dropped before end of \"main\"");
}