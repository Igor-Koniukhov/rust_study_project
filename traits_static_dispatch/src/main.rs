use std::fmt::format;

trait Duplicateable{
    fn dupl(&self) -> String;
}
impl Duplicateable for String{
    fn dupl(&self) -> String {
        format!("{0}{0}", *self)
    }
}
impl Duplicateable for i32{
    fn dupl(&self) -> String {
        format!("{}", *self * 2)
    }
}
fn duplicate<T: Duplicateable>(x: T){
    println!("{}", x.dupl());
}
fn main() {
    let a = 32;
    let b ="Hi their!".to_string();
    duplicate(a);
    duplicate(b);
}
