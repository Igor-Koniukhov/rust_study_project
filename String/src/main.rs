fn main() {
    // String slices
    // String slices are immutable
    let cat1 = "Fluffy";
    let cat: &'static str = "Fluffy";
    println!("Hello, world!, {}, {}", cat1, cat);
    // String objects
    let dog = String::new();
    let mut dog = String::from("Dog");
    println!("{}", dog);
    let owner_says = format!("Hi i'm {} the owner of that {}", "Igor", dog);
    println!("{}", owner_says);
    println!("{}", dog.len());
    dog.push(' ');
    dog.push_str(" the dog");
    println!("{}", dog);
    let new_dog = dog.replace("the", "is my");
    println!("{}", new_dog);


}
