enum IpAddrKind{    //with enum, we can create a variable with one of the values only
    V4,             // as in "let four" and "let six" in the main func below
    V6,
}

struct IpAddr{          //however, with structs, we have to create a variable with all the 
    kind: IpAddrKind,   //struct values initialized
    address: String,
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Message {
    fn call(&self){
        match self{
            Message::Write(value) => println!("{}", value),
            _=> println!("something else"),
        }
    }
}    


struct Color(u8,u8,u8);
struct Point(i32,i32,i32);

enum colors {
    red(Color),
    green(Color),
    blue(Color),
    black(Color),
    white(Color),
}

enum MyEnum{}

impl MyEnum {
    pub const A: i32 = 123;
    pub const B: i32 = 456;
}

enum MyEnum2 {
    A,
    B,
}

impl MyEnum2 {
    fn value(&self) -> i32 {
        match self {
            MyEnum2::A => 123,
            MyEnum2::B => 456,
        }
    }
}
fn main(){

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let m = Message::Write(String::from("hello"));
    m.call();

    let m2 = Message::Quit;
    m2.call();

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("hello world");
    println!("MyEnum A is {}", MyEnum::A);
    println!("MyEnum2 B is {}", MyEnum2::B.value());
}