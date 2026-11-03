fn main() {

     /*
     tuple 
     Issue 1: i8 cannot store 500
     An i8 can store values from -128 to 127.
     Use i16 or i32 instead.

     Issue 2: f8 is not a Rust data type

     Rust supports:
     - f32 – 32-bit floating point
     - f64 – 64-bit floating point
     
     Issue 3: You cannot directly print a tuple using {}
     */

     let tup: (i32,f32,u8) = (500,0.5,5);
     let (x,y,z) = tup;
     println!("{x},{y},{z}");  


     let x: (i32, f64, u8) = (500, 6.4, 1);

     let a = x.0;

     let b = x.1;

     let c = x.2;
      println!("{a},{b},{c}");  
   
}