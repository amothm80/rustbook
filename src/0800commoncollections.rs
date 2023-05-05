use rustbook::indirect_access;


fn main(){
    let number_list = vec![1,2,3,4,5];
    
    let largest = number_list;
    //println!("num 0 is {:?}", number_list); //error is it because of heap?
    println!("largest is {:?}", largest);

    let num_list_arr:[i32;5] = [1,2,3,4,5];
    let larg_arr = num_list_arr;

    println!("num arr 0 is {:?}", num_list_arr); //no error, is it because of stack?
    println!("larg arr is {:?}", larg_arr);

    let ve: Vec<i8> = Vec::new();
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    v.push(6);
    //println!("the first element is {}", first);
    println!("the first element is {}", v[0]);

    for n_ref in &v{
        let n_plus_one: i8 = *n_ref + 1;

        println!("{:p} - {}", n_ref, n_plus_one);
    }

    // for n in v{
    //     let n_plus_one: i8 = n + 1;
    //     println!("{} - {}",n, n_plus_one);
    // }

    for n_ref in &mut v{
        *n_ref += 50;
    }

    let mut vs = Vec::new();

    let ss = String::from("Hello ");
  
    //vs.push(ss);  //does not make line 48 compline due to ownership error
    vs.push(ss.clone());
  
    vs[0].push_str("world");
  
    println!("original: {}", ss);
  
    println!("new: {}", vs[0]);

    let mut v3 = vec![1, 2, 3];

    let mut v4 = Vec::new();
  
    for i in &mut v3 {
  
      v4.push(i);
  
    }
  
    *v4[0] = 5;
  
  
    let a1 = *v4[0];
  
    let b1 = v3[0];
  
    println!("{a1} {b1}");

    indirect_access();

}

