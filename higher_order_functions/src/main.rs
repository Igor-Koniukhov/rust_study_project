fn main() {
    let square = |a: i32| a * a;
    apply(square, 8);
    // Calculate the sum of all the squares less than 500
    // only for even numbers
    let lim = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > lim { break; } else {
            if is_even(isq) {
                sum += isq;
            }
        }
    }
    println!("The sum is {}", sum);

    // With HORs
    let sum2 =
        (0..).map(|x| x * x)
            .take_while(|&x| x <= lim)
            .filter(|x| is_even(*x))
            .fold(0, |sum, x| sum + x);
    println!("The sum using HOFs is {}", sum2);
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("{}", f(a));
}
