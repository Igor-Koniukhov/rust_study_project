use crate::Colors::Red;

fn main() {
   let p1:Point<i32>= Point{X:4, Y:7 };
    println!("{:?}", p1.X + p1.Y);
    let p2:Point<f64>=Point{X:3.45, Y:3.33};
    println!("{:?}", p2);
    let c1 = Red("#f00");
    let c2 = Red(255);
    println!("{:?}", c1);
    println!("{:?}", c2);
    let p3:Point2<i32, f64> = Point2{X:3, Y:4.56};
}


#[derive(Debug)]
struct Point<T>{
    X: T,
    Y: T,
}
#[derive(Debug)]
enum Colors<T>{
    Red(T),
    Green(T),
    Blue(T)
}

struct Point2<T, V>{
    X:T,
    Y:V,
}