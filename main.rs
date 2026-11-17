// Reference and Borrowing

fn main()
{
    let s1 = String::from("Hello");
    let s2 = &s1;
    let s3 = &s1;

    println!("{s2},{s3}");
}

/* ↑ Many readers OR one writer, Mut and Immut ↑ */


//////////////////////////////////////////////////

/* 
fn main()
{
    let mut s = String::from("Rust");
    change(&mut s);

    println!("{s}");
}

fn change(s1:&mut String)
{
    s1.push_str(",aneous");
}

*/
//////////////////////////////////////////////////////

/*  
fn main()
{
    let s1 = String::from("Hello");
    let s2 = s1.len();
 // let s3 = size(s1); // giving ownership to s3 by s1
    let s3 = size(&s1); // no ownership is sharing only borrowing the reference

    println!("{s2},{s3}");
}

fn size(s:&String)->usize
{
    s.len()
}
*/

//////////////////////////////////////////////////////

/* 
fn main()
{
    let s1 = String::from("Hello");
    let s2 = &s1;

    println!("{:p}",s2);
    println!("{s2}");
}

*/