fn main() {
    let numbers = [2,3,4,5,6,7];
    let sl = &numbers[2..4];
    println!("{:?}", sl);
    let mut colors = ["red", "green", "blue", "pink"];
    update_colors(&mut colors[2..4]);
    println!("{:?}", colors)

}

fn update_colors(colors_slice : &mut [&str]){
    colors_slice[0]="yellow";
    colors_slice[1]="orange";
}