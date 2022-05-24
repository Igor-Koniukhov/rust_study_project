fn main() {
   // let primes: Vec<i32> = Vec::new();
    let mut primes = vec![2,3,5];
    primes.push(6);
    primes.remove(0);
    println!("{:?}", primes);
    let mut numbers = vec![3;10];
    const DEFAULT: bool = true;
    let values = vec!(DEFAULT;5);
    numbers[2]=9;
    println!("{:?}, {:?}", numbers, values);
    for number in numbers.iter(){
        println!("{}", number + 3);
    }
}
