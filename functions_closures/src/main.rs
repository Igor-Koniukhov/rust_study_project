fn main() {
    let a = |x: i32, b: i32, c: i32| (x + b) * c;
    println!("{}", a(4, 5, 6));
    let y = |y:i32| -> i32{
        let n = y+7;
        n
    };
    println!("{}", y(5));
    // Closures can be generic type
    let gen = |x| println!("{}", x);
    gen(5);
}
