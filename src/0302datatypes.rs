use std::io;

fn main() {
    //let guess = "42".parse().expect("Not a number!"); //type annotations needed
    let guess: u32 = "42".parse().expect("Not a number!");
    /* integers
    Length	Signed	Unsigned
    8-bit	i8	    u8
    16-bit	i16	    u16
    32-bit	i32	    u32
    64-bit	i64	    u64
    128-bit	i128	u128
    arch	isize	usize
     */
    /* Integer Literals in Rust
    Number literals	Example
    Decimal	        98_222
    Hex	            0xff
    Octal	        0o77
    Binary	        0b1111_0000
    Byte (u8 only)	b'A'
     */

    /* floating points */
    let fx = 2.0; //f64

    let fy: f32 = 3.0; // f32

    /* mathematical operations */
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    /* boolean types */    
    let bt = true;

    let bf: bool = false; // with explicit type annotation

    /* character types */
    let cc = 'z';
    let cz: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    /* compound types */
    /*tuples: fixed length, elements can have multiple types */
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (tx,ty,tz) = tup;

    print!("Second item in the tuple is {ty}\n");

    print!("third item in the tuple is {}\n", tup.2);

    /* compound types */
    /*arrays: fixed length, all elements have same type, collected in the stack not the heap */

    let arr = [1,2,3,4,5];
    print!("third item in the array is {}\n", arr[2]);
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    let arr2: [i32 /*type */; 5/*number of elements */] = [1, 2, 3, 4, 5];
    let arr3 = [3; 5]; /*same as let a = [3, 3, 3, 3, 3] */

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");

}
