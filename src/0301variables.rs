fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
