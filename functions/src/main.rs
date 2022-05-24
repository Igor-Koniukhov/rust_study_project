use std::fmt::format;

fn main() {
    println!("Hello, world!");
    let mut name ="Igor";
    say_hello();
    for i in 1..6{
        say_hello();
    }
    say_hi(&mut name);
   let greet= return_greeting(&mut name);
    println!("{}", greet)
}

fn say_hello(){
    println!("Hello anothe time!");
}

fn say_hi(name: &mut &str){
    println!("Hi my name {}", name);
    *name = "Alex";
    println!("This is my new name {}", name)

}

fn return_greeting(name: &mut &str) -> String{
    let greeting = format!("Hello, I'm greeting you {}", name);
    return greeting;
}
