//this function is used to read integers from console
use std::io;
pub fn readinteger()->i32{

    let mut x =String::new();
    println!("enter an integer");
    let _= io::stdin().read_line(&mut x);
    let age= x.trim().parse().unwrap();  //Parse retuns an enum , to get an integer valued unwrap the enum
    age
   }