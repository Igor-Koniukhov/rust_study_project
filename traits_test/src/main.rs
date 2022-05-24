

struct Pine {
    leaves_color: &'static str,
}

trait Tree {
    fn show_color(&self) -> &str;
    fn grow(&self, m: i32);
}
impl Tree for Pine{
    fn show_color(&self) -> &str {
        self.leaves_color
    }

    fn grow(&self, m: i32) {
        println!(" It's grow on {}", m);
    }
}


fn main() {
    let p = Pine{leaves_color: "green"};
    let red_leaves = Pine{leaves_color: "red"};
    println!("{}", red_leaves.show_color());
   println!("{}", p.show_color());
p.grow(4);

    println!("Hello, world!");
}
