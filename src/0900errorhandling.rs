use std::fs::{self,File};
use std::io;
use std::io::Read;
use std::io::ErrorKind;


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


fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}


fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}



fn main() {
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         ErrorKind::PermissionDenied => panic!("No permission to open file"),
    //         others => {
    //             panic!("Problem opening the file: {:?}", others);
    //         }
    //     },
    // };

    let greeting_file = greeting_file_result.unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error); })
        }else if error.kind() == ErrorKind::PermissionDenied{
            panic!("No permission to open file");
        }else{
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

////the below is correct as well since main is returning a result
// fn main()->Result<(),Box<dyn Error>>{
//     let f = File::open("hello.txt")?;
//     Ok(())
// }