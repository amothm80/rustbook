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

    println!("");
    println!("");

    let d2x: Box<i32> = Box::new(-1);

    println!("d2x is {} at address {:p} pointing to address {:p}", d2x,&d2x,d2x); 
    let x_abs1 = i32::abs(*d2x); // explicit dereference
    let x_abs2 = d2x.abs();      // implicit dereference
    assert_eq!(x_abs1, x_abs2);
    
    let d2r: &Box<i32> = &d2x;
    println!("d2r is {} at address {:p} pointing to address {:p}", d2r,&d2r,d2r);     
    let r_abs1 = i32::abs(**d2r); // explicit dereference (twice)
    let r_abs2 = d2r.abs();       // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);
    
    let d2s = String::from("Hello");
    println!("d2s is {} at address {:p}", d2s,&d2s);     
    let s_len1 = str::len(&d2s); // explicit reference
    let s_len2 = d2s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);
    println!("");
    println!("");

    let q1x = Box::new(0);
    let q1y = Box::new(&q1x);
    println!("q1x is {} at address {:p} pointing to address {:p}", q1x,&q1x,q1x);     
    println!("q1y is {} at address {:p} pointing to address {:p}", q1y,&q1y,q1y);     
    println!("q1y is {} at address {:p} pointing to address {:p}, 
        pointing to address {:p}, pointing to address {:p}", q1y,&q1y,q1y,*q1y, **q1y);

    let mut vec: Vec<i32> = vec![1,2,3];
    let mut vecad : &Vec<i32> = &vec;
    vec.push(4);
    
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let mut num: &mut i32 = &mut vec[2];
    //vec.push(4);
    //vec[2] = 2;
    //println!("Third element is {}", vec[2]); 
    *num = 5;
    println!("Third element is {}", *num); 
    println!("Third element is {}", vec[2]); 
    println!("Vector is now {:?}", vec); 
    
            //undefined behavior: pointer used after its pointee is freed
            //because changing the vector size requires creating a new
            //allocation with a different size which could be in a different
            //place in the memory
            // vec cannot be mutated or dropped while num is in use.

/*Data can be aliased. Data can be mutated. 
But data cannot be both aliased and mutated. 
For example, Rust enforces this principle for boxes 
(owned pointers) by disallowing aliasing. Assigning 
a box from one variable to another will move ownership, 
invalidating the previous variable. Owned data can 
only be accessed through the owner — no aliases.

However, because references are non-owning pointers,
 they need different rules than boxes to ensure the 
 Pointer Safety Principle. By design, references are 
 meant to temporarily create aliases. In the rest of 
 this section, we will explain the basics of how Rust 
 ensures the safety of references through the borrow checker. */            

//References Change Permissions on Paths
//https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html#references-change-permissions-on-paths

/*
The core idea behind the borrow checker is that variables have three kinds
of permissions on their data:

Read (R): data can be copied to another location.
Write (W): data can be mutated in-place.
Own (O): data can be moved or dropped.
These permissions don't exist at runtime, only within the compiler. 
They describe how the compiler "thinks" about your program before the 
program is executed.

By default, a variable has read/own permissions (RO) on its data. 
If a variable is annotated with let mut, then it also has the 
write permission (W). The key idea is that references can 
temporarily remove these permissions.
 */


    let mut s = String::from("hello");
    let hello: &str = &s[0..5];
//    s = String::from("World");
    println!("{hello}");
    s.push_str(" world");

    let xi8 : i8 = 1; //not moved
let xu128 : u128 = 1; //not moved
let xis : isize = 1;//not moved
let xf64 : f64 = 1.0;//not moved
let xbool : bool = true;//not moved
let xchar : char = 'a';//not moved
let xstr : &str = "a";//not moved
let xt : (i32,i32) = (1,1);//not moved
let xai : [i32;3] = [1,2,3];//not moved
let xas : [String;3] = [String::from("a"), String::from("b"), String::from("c")];//moved
let s : String = String::from("a");//moved
let b : Box<i8> = Box::new(1);//moved

//let _mv = b;

println!(
"{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
xi8,xu128,xis,xf64,xbool,xchar,xstr,xt,xai,xas,s,b
);

}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);    
}


//We now have a way to find out the index of the end of the 
//first word in the string, but there’s a problem. We’re 
//returning a usize on its own, but it’s only a meaningful 
//number in the context of the &String. In other words, 
//because it’s a separate value from the String, there’s 
//no guarantee that it will still be valid in the future
fn first_word_bad(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


//Now when we call first_word, we get back a single value that is 
//tied to the underlying data. The value is made up of a reference
//to the starting point of the slice and the number of elements in the slice.
fn first_word_good(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}