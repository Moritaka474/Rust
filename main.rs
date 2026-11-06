fn main()
{   
    let a = [1,2,3,4,5];
    let mut sum = 0;
    for x in 0..5
    {
        sum = sum+a[x];   
        
    }
    println!("{sum}");
}

/////////////////////////////////////////////////////
// Rust for loop syntax
/* 
// Exclusive range (end value excluded)
for x in 0..5 {
    println!("{x}");  // 0, 1, 2, 3, 4
}

// Inclusive range (end value included)
for x in 0..=5 {
    println!("{x}");  // 0, 1, 2, 3, 4, 5
}

// Reverse loop
for x in (0..=5).rev() {
    println!("{x}");  // 5, 4, 3, 2, 1, 0
}
*/

//////////////////////////////////////////////////

/*
fn main()
{
    for number in (1..4).rev()
    {
        println!("{number}");
    }
    println!("TAKE OFF!!!");
}
  
*/

////////////////////////////////////////
/*
fn main()
{
    let a = [1,2,3,4,5];
    for element in a
    {
        println!("Values : {element}");
    }
}
 */




