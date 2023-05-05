#[allow(unused)]
fn main(){
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    println!("the value of {} is {:?}","Yellow", scores.get(&String::from("Yellow")).unwrap());
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
