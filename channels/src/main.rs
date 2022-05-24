use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use crate::message_receiver::mr::{NUM_THREADS, message_receiver};
mod message_receiver;


fn start_thread(d: usize, tx: mpsc::Sender<usize>){
    thread::spawn(move||{
        println!("setting timer {}", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("sending {}", d);
        tx.send(d).unwrap()
    });
}

fn main() {
    // Channels a way to send data between threads
    // MPSC - multiple producer single receiver
   /*let (tx, rx)=mpsc::channel();
    thread::spawn(move ||{ tx.send(42).unwrap() });
    println!("received {}", rx.recv().unwrap())*/
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_THREADS{
        start_thread(i, tx.clone());
    }
  message_receiver(rx);

}


