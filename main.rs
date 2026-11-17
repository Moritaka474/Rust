// Dangling pointer
/*
**Definition of a Dangling Pointer**

*   **Invalid Reference:** A pointer that points to a memory location that has been **deallocated** or **no longer valid**.
*   **Origin:** Created when the object pointed to is deleted or goes out of scope, but the pointer’s value is not updated (e.g., set to `NULL`).
*   **Risk:** Accessing or dereferencing a dangling pointer leads to **undefined behavior**, such as crashes, data corruption, or security vulnerabilities.
*   **Common Causes:**
    *   Returning a reference to a local variable.
    *   Using memory after it has been freed (`free`/`delete`).
    *   Using a pointer after the object it references has been destroyed.

*/


fn main()
{
    let x = y();
    println!("{x}");
}

fn y()->String             // instead of return the borrow value just pass the ownership that way it works
{
    let z = String::from("Hello");
    z
}