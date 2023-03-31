fn main() {
    let number = 7;
    /*if else */
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //if number { //wrong
    if number != 0 {
        //right
        println!("number was seven");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = false;
    let number = if condition { 5 } else { 6 }; //if expression has to be boolean, no other type is accepted
                                                //let number = if condition { 5 } else { "six" }; //wrong, both arms of the if condition
                                                //have to have the same  return type if the are assigned directly

    println!("The value of number is: {number}");

    /* loops: loop, while, for*/
    /*loop will loop forever until a break */
    let mut counter = 0;

    let mut result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The loop1 result is {result}");

    loop {
        counter += 1;
        if counter == 20 {
            result = counter;
            break;
        }
    }

    println!("The loop2 result is {result}");
    /*loop labels */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    /*while loop */
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let whilearr = [10, 20, 30, 40, 50];
    let mut windex = 0;

    while windex < 5 {
        println!("the value in while array is: {}", whilearr[windex]);

        windex += 1;
    }


/* for loop */

    let fora = [10, 20, 30, 40, 50];

    for forelement in fora {
        println!("the value in for array is: {forelement}");
    }    

    for fornumber in (1..4).rev() {
        println!("{fornumber}!");
    }
    println!("LIFTOFF!!!");

}
