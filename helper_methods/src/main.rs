use std::fs::File;
fn main() {
  // let f = File::open("main.jpg");
   // match f {
     //   Ok(f)=>{
      //      println!("file found {:?}", f);
       // }
        //Err(e)=>{
          //  panic()
        //}
    //}

    //let f = File::open("main.jpg").unwrap();
    let f = File::open("main.jpg").expect(" Unable open the file");
}
