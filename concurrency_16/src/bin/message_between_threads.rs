use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

fn main(){
    let(tx, rx) = mpsc::channel();
    let tx1 = Sender::clone(&tx);
    thread::spawn(move ||{
        let val = vec![
            String::from("Input from rec 1 :1"),
            String::from("Input from rec 1 :2"),
            String::from("Input from rec 1 :3"),
            String::from("Input from rec 1 :4"),
            String::from("Input from rec 1 :5"),
        ];
        
        for val in val{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    thread::spawn(move ||{
        let val = vec![
            String::from("Input from rec 2 :1"),
            String::from("Input from rec 2 :2"),
            String::from("Input from rec 2 :3"),
            String::from("Input from rec 2 :4"),
            String::from("Input from rec 2 :5"),
        ];
        
        for val in val{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("Got: {received}");
    }
    
    
}