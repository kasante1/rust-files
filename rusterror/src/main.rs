use std::fs::File;
//use std::io::ErrorKind;
use std::io;
use std::io::Read;



fn main() {
    // let f = File::open("/home/asante/Desktop/nlpguide.txt").unwrap();

    // let f = File::open("/home/asante/Desktop/nlpguide.txt").expect("Failed to open hello.txt");


    
    // let f = match f{
    //     Ok(file) => file,
    //     Err(error) =>match error.kind(){
    //         ErrorKind::NotFound => match File::create("/home/asante/Desktop/hello.txt"){
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("problem opening the file: {:?}", other_error)
    //     },
    // };
    // println!("{:?}", f);


    //Propagating Errors


    fn read_username_from_file() -> Result<String, io::Error>{
    let f = File::open("/home/asante/Desktop/hello.txt");
    let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
    };
    let mut s = String::new();

    match f.read_to_string(&mut s){
    Ok(_) => Ok(s) ,
    Err(e) => Err(e) ,
    }
    }
    let h = read_username_from_file();

    println!("{:?}", h);

    //A Shortcut for Propagating Errors: the ? Operator

    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut f = File::open("hello.txt")?;
    //     let mut s = String::new();
    //     f.read_to_string(&mut s)?;
    //     Ok(s)
    //     }
        
   // a way to make this even shorter.

    // use std::io;
    // std::fs;
    // fn read_username_from_file() -> Result<String, io::Error> {
    // fs::read_to_string(“hello.txt”)
}

