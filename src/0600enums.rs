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

enum UsState {
    Alabama,
    Alaska,
    NewYork,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 
            match state {
                UsState::Alaska => 24,
                UsState::NewYork => 26,
                _ => 25,
            }
    }
}

fn coin_pnd(b:bool) -> Coin{
    if b{
        Coin::Penny
    }else{
        Coin::Quarter(UsState::NewYork)
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

    let some_number = Some(5); //Some<T>
    let some_char = Some('e'); //Some<T>
    
    let absent_number: Option<i32> = None;    

    let text: Option<String> = Some("Hello, world!".to_string());
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `text` on the stack.
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    //text.map(|s| s.len()); //this wont work because it will consume text
    println!("still can print text: {text:?}");

    println!("value of penny in cents is {}", value_in_cents(coin_pnd(true)));
    println!("value of quarter in cents is {}", value_in_cents(coin_pnd(false)));

    let x = 1;
    match x{
        1 => println!("1"),
        2 => println!("2"),
        other => println!("{}", other), //catch all with value
        //_ => println!("others"), //catch all without value
    }

    let opt: Option<String> =  Some(String::from("Hello world"));
    match &opt {    //this borrows opt
        // _ became s
        Some(s) => println!("Some: {}", s),
        None => println!("None!")
    }; 

    println!("{:?}", opt); //this is fine because opt is borrowed

    match opt { //this move opt
        // _ became s
        Some(s) => println!("Some: {}", s),
        None => println!("None!")
    };   
        
    //println!("{:?}", opt); //this is not fine because opt is moved
}