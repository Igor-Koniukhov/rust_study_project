pub mod mr {
    use std::sync::mpsc::Receiver;

    pub const NUM_THREADS: usize = 20;

    pub fn message_receiver(rx: Receiver<usize>) {
        for j in rx.iter().take(NUM_THREADS) {
            println!("received {}", j);
        }
    }
}
