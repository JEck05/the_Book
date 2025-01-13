use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main(){
    // Arc is a thread safe version of Rc 
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter_ref = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = counter_ref.lock().unwrap();
            *num += 1;

        });
        handles.push(handle);
    }
    
    for thread in handles{
        thread.join().unwrap();
    }
    println!("result = {}", counter.lock().unwrap());
}