use std::fmt::format;

trait Bark {
    fn bark(&self) -> String;
}

struct Dog {
    species: &'static str,
}

struct Cat {
    color: &'static str,
}

impl Bark for Dog {
    fn bark(&self) -> String {
        return format!("{} ,barking", self.species);
    }
}

fn bark_it<T: Bark>(b: T) {
    println!("{}", b.bark())
}


fn main() {

    let small_dog = Dog{species: "small dog"};

    let cat = Cat { color: "black" };
    bark_it(small_dog)
}
