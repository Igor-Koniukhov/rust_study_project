use std::io;
/// Create comment.
/// What is this module truing to achieve
fn main() {
    //! # Main function
    //! ```
    //! fn main()
    //! ```
    //! Reads user input and prints it to the console.
    //!
    let mut input=String::new();

    println!("Say something!");
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("You said {}", input);

        },
        Err(e)=>{
            println!("Something went wrong {}", e);
        }
    }
}
