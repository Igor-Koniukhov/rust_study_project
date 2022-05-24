
macro_rules! my_macro{
    () => (println!("First macro"))
}
macro_rules! name {
    ($name: expr) => {println!(" Hey {}", $name)}
}
macro_rules! name2 {
    ($($name: expr), *) => ($(println!(" Hey {}", $name);)*)
}
macro_rules! xy{
    (x => $e: expr) => (println!("X is {}", $e));
    (y => $e: expr)=> (println!("Y is {}", $e));
}
macro_rules! build_fn{
    ($fn_name: ident)=>{
        fn $fn_name(){
            println!("{:?} was called", stringify!($fn_name));
        }
    }
}

fn main() {
   my_macro!();
    name!("Igor");
    name2!("Igor", "Alex", "Boris");
    xy!(x => 8);
    xy!(y => 10);
    build_fn!(hey);
    hey();
}
