use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let c = Arc::new(Mutex::new(0));
    let mut threads = vec![];
    for i in 0..10{
        let c = Arc::clone(&c);
        let t = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num +=1;
        });
        threads.push(t);
    }
    // Waiting for all threads which we started
    for th in threads{
        th.join().unwrap();
    }
    println!("Result {}", *c.lock().unwrap());
}
