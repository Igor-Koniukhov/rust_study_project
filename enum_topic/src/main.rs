use crate::Colors::Red;
use crate::Person::Name;

fn main() {
    let my_colors = Colors::Red;
    println!("{:?}", my_colors);
    let my_colors = Red;
    println!("{:?}", my_colors);
    let person = Name(String::from("Alex"));
    println!("{:?}", person);
}

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}
#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(u32),
}
