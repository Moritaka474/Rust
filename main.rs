/*
    Functions with Return Values

    If you add a semicolon to the end of an expression, you turn it into a statement, 
    and it will then not return a value. 
    Keep this in mind as you explore function return values and expressions next.
*/
use std::io;
fn main()
{
   let mut x = String::new();
   print!("Enter the value: \n");
   io::stdin().read_line(&mut x).expect("Error");

   let x = x.trim().parse().expect("Error"); 
   let a = square(x);
   println!("{a}");

   let area = rectangle(5, 5);
   println!("{area}");

} 

fn square(x:u32)->u32
{
   let result =   x*x;
   result 

}

fn rectangle(width:u32,length:u32)->u32
{
    width * length
}

