fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    let x = true;
    read(x); // oh no! x isn't defined!    
    let a = Box::new([0, 1, 2, 3, 4, 5]);
    let b = a; // change of box ownership from a to b. a cannot be used to access box anymore
    //print!("{}", a[0]); //moved heap pointer, a has no access anymore
    println!("{}", b[0]); 
    //b[0] = 6;
    println!("{}", b[0]); 
  
    let first = String::from("Ferris");
    let first_clone = first.clone(); //string ferris has been cloned, first has a copy and first_clone has
                                    //another copy
    let full = add_suffix(first_clone); //first clone will go out of scope and heap ownership will be 
                                        //terminated. due to the result of fn add_suffix, the ownership
                                        //will be transferred to variable full
    println!("{full}, originally {first}");

    let c = Box::new([0, 1, 2, 3, 4, 5]);
    let d = &c;
    println!("{}", c[0]);
    println!("{}", d[0]); 
/* references */    
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let _s = format!("{} {}", m1, m2);
    

/*dereferencing */
//https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html#dereferencing-a-pointer-accesses-its-data

    let mut dx: Box<i32> = Box::new(1);
    println!("dx is {} at address {:p} pointing to address {:p}", dx, &dx, dx);
    //dx = dx + 1; //value cannot be referenced directly
    let mut ax: i32 = *dx; //example of copying value to another var by dereferencing
                    // *dx reads the heap value, so ax = 1
    *dx += 1; //example of value processing by dereferencing
            // *dx on the left-side modifies the heap value, 
            //     so dx points to the value 2
             
    println!("dx is {} at address {:p} pointing to address {:p}", dx, &dx, dx);

    let dr1: &Box<i32> = &dx;  // dr1 points to dx on the stack
    println!("dr1 is {} at address {:p} pointing to address {:p}", dr1,&dr1,dr1);    
    let db: i32 = **dr1;       // two dereferences get us to the heap value
    let dr2: &i32 = &*dx;      // r2 points to the heap value directly
    println!("dr2 is {} at address {:p} pointing to address {:p}", dr2,&dr2,dr2);        
    let dc: i32 = *dr2;    // so only one dereference is needed to read it


}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
}