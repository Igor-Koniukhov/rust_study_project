// Only one variable can own a piece of memory
// Varibles can borrow ownership to other pieces of memory

fn main() {
    let mut a = 5;
    {
        let b = &mut a;
        println!("b = {} ", *b);
        *b += 2;
    }
    println!(" a = {}", a);


let mut v = vec![1,2,3,4,5];
    for i in &v{
        println!("{}", i);
    }

}
