/*
    Ownership
*/

/*
==================================================
             RUST OWNERSHIP - NOTES
==================================================

1. STRING LITERAL (&str)
-------------------------
- String literals have a fixed size.
- Their text is stored in the program binary
  (typically in read-only memory).
- They cannot be modified or extended.

Example:
    let s = "hello";


2. STRING TYPE (String)
-------------------------
- String is a growable, mutable string type.
- Its text is stored in heap memory.
- It can be extended using push_str().

Example:
    let mut s = String::from("hello");
    s.push_str(" Rust");


3. STRING MEMORY STRUCTURE
---------------------------
A String variable contains three pieces of metadata:

- Pointer  : Address of the heap data.
- Length   : Current length of the text.
- Capacity : Allocated memory capacity.

The actual text is stored in heap memory.


4. OWNERSHIP AND MOVE
----------------------
Example:
    let s1 = String::from("hello");
    let s2 = s1;

- Ownership moves from s1 to s2.
- Rust does not copy the actual heap data.
- The pointer, length and capacity are moved.
- Both variables do not remain valid owners.
- After the move, s1 cannot be used.

This is called MOVE semantics.


5. COPY TRAIT
---------------
Example:
    let x = 10;
    let y = x;

- Integer types such as i32 and u32 implement Copy.
- Rust automatically copies their values.
- Both x and y remain usable.
- Each variable has its own independent value.

Other common Copy types:
    bool
    char
    floating-point types


6. CLONE METHOD
-----------------
Example:
    let s1 = String::from("hello");
    let s2 = s1.clone();

- clone() creates a separate String.
- It copies the heap data into a new allocation.
- Both s1 and s2 remain usable.
- Each String owns its own heap allocation.

Note:
- clone() is generally unnecessary for simple Copy types.


7. DROP FUNCTION
------------------
- Rust automatically calls drop() when an owned
  value goes out of scope.
- drop() releases the resources owned by that value.
- A moved-from variable does not release the
  transferred resource.
- The current owner releases it when its scope ends.


8. DOUBLE-FREE ERROR
---------------------
- Double-free happens when the same memory
  allocation is released more than once.
- It can cause heap corruption, program crashes,
  unpredictable behaviour and security issues.
- Rust ownership prevents ordinary double-free
  mistakes in safe Rust code.


9. IMPORTANT OWNERSHIP RULE
-----------------------------
- Each value has one owner at a time.
- When the owner goes out of scope, the value
  is dropped.
- Ownership can be transferred (MOVE).
- Values implementing Copy are copied instead.
- clone() explicitly creates a separate copy.

==================================================
                  END OF NOTES
==================================================
*/




/*
    Return Values and Scope

    fn main()
{
    let s1  = gives_ownership();
    println!("{s1}");
}

fn gives_ownership() -> String
{
    let some_string = String::from("Hello");
    some_string
}
*/

/////////////////////////////////////////////

/*
    Ownership and Functions
    
    fn main()
{
    let s = String::from("Hello");
    take_ownerhsip(s);

    let x = 5;
    make_copy(x)
}

fn take_ownerhsip(s:String)
{
    println!("{s}");
}

fn make_copy(x:u32)
{
    println!("{x}");
}

*/

///////////////////////////////////////////////

/*
fn main()
{
    let  s1 = String::from("hello");
    let  s2 = s1.clone();

    println!("{s2}, world!");

}

*/

