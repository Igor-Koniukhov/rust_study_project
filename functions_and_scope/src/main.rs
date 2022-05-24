
static mut B :i32= 4;
fn main() {
    unsafe {println!("{}", B)}
    {
        let a = 78;
        println!("{}", a)
    }
}
