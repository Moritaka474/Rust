use std::io;
fn main()
{
    let mut x = String::new();
    println!("Enter the value: ");
    
    io::stdin().read_line(&mut x).expect("Error");

    let x:i8 = x.trim().parse().expect("Invalid number");

    if x > 0 
    {
        println!("Postive");
    }
    else if x==0
    {
       println!("Zero");    
    }
    else 
    {
         println!("Negative");
    }

}
