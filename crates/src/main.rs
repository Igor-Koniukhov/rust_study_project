//use crate::archive::arch::arch_files;
use crate::archive::arch::arch_files as arc;
use rand::Rng;
mod archive;

fn main() {
    println!("Hello, world!");
    arc("somefile.txt");
    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    println!("{}", a);
}
