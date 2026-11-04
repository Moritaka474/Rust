/*
     User Input, where first take variable input as string without declaring datatype, 
     then convert string to integer with parse(). 

*/

use std::io;
fn main()
{
    let mut length = String::new();
    let mut width = String::new();
    let area;

    println!("Enter the width: ");
    io::stdin().read_line(&mut width).expect("Invalid number");

    println!("Enter the length: ");
    io::stdin().read_line(&mut length).expect("Invalid number");

    let width: u32 = width.trim().parse().expect("Error");
    let length: u32 = length.trim().parse().expect("Error");

    area = width * length;
    println!("Area of rectangle is: {area}");



}