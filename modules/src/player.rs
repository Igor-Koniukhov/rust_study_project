use crate::clean::files::clean_files;
use crate::clean::perform_cleaner;

pub fn play_movie(name: &str){
    println!("Playing movie {}", name);
    perform_cleaner();
    clean_files()
}
pub fn play_audio(name: &str){
    println!("Playing audio {}", name);
    wr::wr_some_logic();

}

pub mod wr{
    pub fn wr_some_logic(){
        println!("Some logic")
    }
    pub mod read{
        pub fn read(){
            println!("Some read")
        }
    }

}