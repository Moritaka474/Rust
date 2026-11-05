/*
    loop
*/
//------------------------------------------------//
/* 
use std::io;
fn main()
{
    let mut x = String::new();
    println!("Enter the value: ");

    io::stdin().read_line(&mut x).expect("Error");
    let x = x.trim().parse().expect("Error");

    let mut counter = 1;
    loop 
    {
       println!("{counter}");
       if counter == x
       {
          break;
       } 
       counter+=1;
          
    }
}
*/

// Returning Values from Loops
/* 

fn main()
{
    let mut counter = 0;
    let result = loop {
        counter+=1;
        if counter == 10
        {
            break counter + 2;
        }
    };
    println!("{result}");
}

*/