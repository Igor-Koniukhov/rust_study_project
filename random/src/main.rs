use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
fn main() {

    let mut rng = thread_rng();
    for a in 1..5{
        let i: i32 = rng.gen();
        println!("position: {}, {}", a, i )
    }


    println!("bounded int: {}", rng.gen_range(0..100));
    println!("bounded float: {}", rng.gen_range(0.0..100.0));


}
