use std::fs::{self,File};
use std::io;
use std::io::Read;

fn read_file()->Result<String, io::Error>{
    let mut s = String::new();
    let mut f=File::open("hello.txt")?;//if file found, function execution will continue
                                        //if not found, function will end and
                                        //Result<io:Error> will be returned
    f.read_to_string(&mut s)?;

    /*
    the above can be simplified as
    File::open("hello.txt")?.read_to_string(&mut s)?;
     */
    Ok(s)
}

fn read_file_from_crate_fs()->Result<String, io::Error>{
    fs::read_to_string("hello.txt")

}
fn main(){
    let f = read_file();
    println!("{:?}",f);
}

////the below is correct as well since main is returning a result
// fn main()->Result<(),Box<dyn Error>>{
//     let f = File::open("hello.txt")?;
//     Ok(())
// }