use std::hash::Hash;

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


    let mut h: HashMap<char, Vec<usize>> = HashMap::new();

    for (i, c) in "hello!".chars().enumerate() {
  
      h.entry(c).or_insert(Vec::new()).push(i);
  
    }
  
    let mut sum = 0;
  
    for i in h.get(&'l').unwrap() {
  
      sum += *i;
  
    }
  
    println!("{}", sum);
}
