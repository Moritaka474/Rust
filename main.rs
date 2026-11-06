fn main()
{
    let a = [1,2,3,4,5];
    let mut index = 0;

    while index < 5
    {
        println!("Value of array : {}",a[index]);
        index+=1;
    }
}


/////////////////////////////////////////////////
/*
fn main()
{
    let mut x = 1;
    while x <= 20
    {
        if x%2 == 0
        {
            println!("Even counted = {x}");
            
        }
          if x%2 != 0
        {
            println!("Odd counted = {x}");
            
        }
         x+=1;
    }

}

*/

/////////////////////////////////////////

/*
fn main()
{
    let mut count = 0;
    'counting_up: loop
    {
        println!("count = {count}");
        let mut remain = 10;
        loop
        {
            println!("remaining = {remain}");
            if remain == 9
            {
                break;
            }
            if count == 2
            {
                break 'counting_up;
            }
            remain-=1;
        }
        count+=1;
    }
    println!("End count = {count}");
}
*/