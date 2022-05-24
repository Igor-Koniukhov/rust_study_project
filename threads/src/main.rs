use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[allow(unused_variables)]
fn main() {
    let mut threads =vec![];
    for i in 0..10 {
// Create thread. move - needs for moving variable i into the scope
      let th =  thread::spawn(move || {
            // sleep a thread
            sleep(Duration::from_millis(i * 1000));
            println!("new thread: {}", i)
        });
        threads.push(th);
    }
    for t in threads{
        t.join().expect("problem with thread");
    }
    println!("main tread")
}
