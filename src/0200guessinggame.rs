use rand::Rng;
use std::cmp::Ordering;
use std::io;

//write a function to print an immutable variable by reference

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret numnber is: {secret_number}");
    loop {
        println!("please input your number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //let guess: u32 = guess.trim().parse().expect("please type in a number");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win");
                break;
            }
        }
    }
}
