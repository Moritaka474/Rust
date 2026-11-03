/*
This is an example of Rust’s memory safety principles in action. 
In many low-level languages, this kind of check is not done, 
and when you provide an incorrect index, invalid memory can be accessed. 
Rust protects you against this kind of error by immediately exiting instead of allowing the memory access 
and continuing.

Array
*/
use std::io;
fn main()
{
     let a = [1,2,3];
     println!("Enter the array index: ");

     let mut index = String::new();

     io::stdin()
       .read_line(&mut index)
       .expect("Fail to read the line");

     let index: usize = index
       .trim()
       .parse()
       .expect("Index is not number");

     let element = a[index];
     println!("Element = {element}");


}

