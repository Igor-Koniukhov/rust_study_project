
fn main() {
   let i = 5;
    let j = i;
    println!("{}", i);
    println!("{}", j);
    let v = vec![1,2,3,4,5];
    println!("{:?}", v);
    let foo = |v: Vec<i32>| -> Vec<i32>{
        println!("Vector used in foo");
        v
    };
    let v2 = foo(v);
    println!("{:?}", v2);
}
