use rand::Rng;

fn main() {
   let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..10);
    if num > 5{
        println!("Number {} is > or == to 5", num);
    }else if num==5{
        println!("Number {} is = 5", num);
    }else{
        println!("Number {} is smaller than 5", num);
    }
}
