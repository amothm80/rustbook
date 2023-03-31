fn main() {
    println!("Hello, world!");
    let mut num = 6;
    another_function(5);
    reference_function(&num);
    print_labeled_measurements(5, 'h');
    let y = {
        let x = 3;
        x + 1
    };
    println!("the value of y is {y}");
    println!("the value of five() is {}", five());
}

fn another_function(x:i32){ /*parameter definition; varname:type */
    println!("value of parameter is {x}");
}

fn reference_function(mut y:&i32){ /*referenced parameter definition; varname:&type */
    println!("value of reference is {y}");
    y = &10;
}

fn print_labeled_measurements(value: i32, unit_label: char){
    println!("The measurement is : {value}{unit_label}");
}

fn five() -> i32{
    5
}