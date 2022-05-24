#[allow(unused_variables)]
fn main() {
    println!("Hello, world!");
    println!("Hi my name is {} and i'm {}", "Igor", 42);
    println!("a + b = {}", 3 + 6);
    println!("{0} has {2} and {1}", "Igor", "dog", "cat");
    println!("{name} {surname}", surname = "koniukhov", name = "Igor");
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 5, 5, 5);
    // :? - debug trait
    println!("{:?}", [1, 2, 3]);
    let name = "Igor";
    let mut age = 42;
    let amount: i64 = 45654333344;
    age = 43;
    println!("{},{},{}", name, age, amount);

    // Shadowing is allowed
    let color = "blue";
    let color = "red";
    println!("{}", color);

    // Allowed declare simultaneously multiple variables
    let (a, b, c) = (3, 4, "red");
    println!("{},{},{}", a, b, c)
}
