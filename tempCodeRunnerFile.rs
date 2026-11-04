use std::io;
fn main()
{
   let mut x = String::new();
   print!("Enter the value: \n");
   io::stdin().read_line(&mut x).expect("Error");

   let x = x.trim().parse().expect("Error"); 
   let a = square(x);
   println!("{a}");

} 

fn square(x:u32)->u32
{
    x*x
}

