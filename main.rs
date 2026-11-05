use std::io;
use rand::prelude::*;
fn main()
{
    let mut counter: u32 = 0;
    let mut x = String::new();
    println!("Enter the repeatation: ");

    io::stdin().read_line(&mut x).expect("Error");
    let x = x.trim().parse().expect("Error");    
    let mut sum: u32 = 0;
    loop
    {
       let rng = rand::rng().random_range(1..=10);
        println!("{rng}");
        sum = sum+rng;
        counter+=1; 
       if counter == x
       {   
           println!("Sum of rng is: {sum}");
           break;
       }
          
    };
}
