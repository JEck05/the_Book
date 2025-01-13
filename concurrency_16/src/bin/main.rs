use std::thread;
use std::time::Duration;

fn main() { 
    capture_variables_threads();
    
    let handle = thread::spawn(|| {
        for i in 1..10{
            println!("Num: {i} from Spawned thread");
            thread::sleep(Duration::from_millis(10));
        }
    });
    // If we were to uncomment the below line
    // it would print all the handle threads output before it 
    // approaches the next for loop
    // handle.join().unwrap();
    for i in 1..5{
        println!("Num: {i} from Main thread");
        thread::sleep(Duration::from_millis(1));
    }
    
    // Joining threads causes the current one(Main) to wait for the 
    // thread to end before it does
    handle.join().unwrap();
    
}

fn capture_variables_threads(){
    let v = vec![2, 5, 3];
    
    // move keyword makes it so v is moved to the closure inside 
    // handle
    let handle = thread::spawn(move || {
       for i in v{
           println!("{i}");
       } 
        
    });
    
    handle.join().unwrap();
}
