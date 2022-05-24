use std::fmt::format;

trait Duplicateable {
    fn dupl(&self) -> String;
}

impl Duplicateable for String {
    fn dupl(&self) -> String {
        format!("{0} {0}", *self)
    }
}

impl Duplicateable for i32 {
    fn dupl(&self) -> String {
        format!("{}", *self * 2)
    }
}

fn duplicate(x: &dyn Duplicateable) {
    println!("{}", x.dupl());
}

fn main() {
    let a = 34;
    let b = "Hi John! ".to_string();
    duplicate(&a);
    duplicate(&b);
}
