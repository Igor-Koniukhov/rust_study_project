fn main() {
    let mut person = ("John", 44, true);
    println!("{:?}", person);
    person.1 = 55;
    let (name, age, employed)=person;
    println!("{:?}", name);
}
