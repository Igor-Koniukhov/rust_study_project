use crate::Suit::{Heart, Spade, Club, Diamond};

fn main() {
    print_choice(Heart);
    print_choice(Spade);
    print_choice(Club);
    print_choice(Diamond);

    country(44);
    country(34);
    country(234);
    country(-12);
}

fn country(code: i32){
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1..=999 => "unknown",
        _ => "invalid"
    };
    println!("Country is {}", country);
}

enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
}

fn print_choice(choice: Suit) {
    match choice {
        Heart => { println!("\u{2665}") }
        Spade => { println!("\u{2660}") }
        Club => { println!("\u{2663}") }
        Diamond => { println!("\u{2666}") }
    }
}