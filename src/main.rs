mod arithmethods;  //Imprting the visibility of ArithMethods module to main.rs module
mod vectors;
mod tuples;
fn main() {
    println!("Hello, world!");
    let mut x = 10;   //If you wish assign value to varaible for more than once in a program, make it mutable 
    let mut y = 32;

    println!("sum of x + y ={}", x+y);   

    x = 43;
    y = 34;

    println!("sum of x + y ={}", x+y) ;

    println!( "8%64 {}", 8%64);
    //Find average of three numbers
    let a =13;
    let b=2.3;
    let c=120.0;
    let average = (a as f64 + b + c)/3.0;
    assert_eq!(average,45.1);
    println!("avera of {}, {}, {} = {}",a,b,c,average);

    //tuple example 
    let car = ("red","tayota",1000,26,2.5);
    println!("Car Color : {}",car.0);
    let student = (124567,9,"vijaya Sekhar");
    tuples::displaystudent(student);
   //Array of Tuples pending 
   let students = [(124567,9,"vijaya Sekhar"),(124565,11,"Sravani"),(124564,12,"Sahithi")];
    for i in students
    {
     tuples::displaystudent(i);
    }

    //Array for tuples
    let car2 = [("red","tayota",1000,26,2.5),("blue","tayota",1000,26,2.5)];
    println!("2nd Car Color : {}",car2[1].0);

    println!("Moving to functions");
    //find GCD of two numbers 
      let gcdvalue = gcd(8,64);
      println!("GCD of 64, 8 = {}",gcdvalue);   


//call sum functionf rom ArithMethod module
println!(" I am calling Asub method from Arith module {}", arithmethods::asum(10,20));

//call popualte vector 
 let mut vec:Vec<i32>= Vec::<i32>::new();
 vec= vectors::pupulate(vec, 10);
 
//call displayvector
vectors::displayvector(vec);

//determine element existance , error : value used here after move
//if vectors::elementexists(vec, 10)
//{
//    println!("element 10 exists in vector vec");
//}
//else {
  //  println!("element 10 does not exists in vector vec");
//}


//Arrays 
let marks = [1,2,3,4,5];   // an array for 5 integers of size i32 


}


fn gcd(mut n : u64, mut m:u64 ) ->u64{
    assert!(n !=0 && m !=0);
   while m!= 0{
    if m < n 
    {
        let t=m;
        m =n;
        n=t;
    }
    m= m%n;
}
n
}




//Test function for GCD
#[test]
fn test_gcd()
{
    assert_eq!(gcd(14,15),1);
    assert_eq!(gcd(199,3),1);
    assert_eq!(gcd(64,8),8);
    println!("I am in test function before false postive case");
    assert_eq!(gcd(1,1),3);
}