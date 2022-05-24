mod player;


fn main() {
    println!("Hello, world!");
    player::play_movie("snatch");
    player::play_audio("music.mp3");
    clean::perform_cleaner();
    clean::files::clean_files();
    player::wr::wr_some_logic();
    player::wr::read::read();

}

mod clean{
    pub fn perform_cleaner(){
        println!("Clenaing hdd");
    }
    pub mod files{
        pub fn clean_files(){
            println!("Removing unused files!")
        }
    }
}