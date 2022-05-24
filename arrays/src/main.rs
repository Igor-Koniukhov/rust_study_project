#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let primes = [2, 3, 5, 7, 8];
    let doubles: [f64; 4]=[0.3, 0.5, 0.7, 0.9];
    println!("{:?}", primes);
    println!("{:?}", doubles);
// Creating array with default values
    let numbers = [0;15];
    println!("{:?}", numbers);

    const DEFAULT: i32 = 3;
    let mut numbers = [DEFAULT; 15];
    println!("{:?}", numbers);
    // Updating elements
    numbers[3]=7;
    println!("{:?}", numbers);

    // Using an iterator
    for number in numbers.iter(){
        println!("{}", number + 3);
    }
    le

}
