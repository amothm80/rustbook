#[allow(unused)]
fn main(){
    let mut s = String::new();
    let data = "initial content";
    let s = data.to_string();
    let s = "initial content".to_string();
    let hello = String::from("Здравствуйте");
    
    println!("hello contents {}",hello);
    println!("the length of the hello grapehemes {:?}",hello.chars().count());
    //for c in hello.chars() {
    //        println!("{}", c);
    //}
    
    println!("hello len is {}", hello.len());
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    
}

