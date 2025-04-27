// Ownership rules
// 1. Each value in Rust has a variable that's called its owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of the scope, the value is dropped
fn main() {
    // Variable Scope
    // As a first example of ownership, let's look at the scope of variables. A scope is the range within a program for which an item is valid. Here's a simple example:
    {                           // s is not valid here
        let s = "hello";  // s is valid from this point forward
        println!("{s}")         // do stuff with s
    }                           // this scope is now over, and s is no longer valid
    // When s comes into scope, it is valid. It remains valid until it goes out of scope.
    // The String Type
    // To illustrate ownership more concretely, let's look at a more complex data type: String. String literals like "hello" are convenient but immutable. The String type, on the other hand, is allocated on the heap and can store text that we don't know the size of at compile time.
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
    //The String type is different from string literals because it can be mutated, whereas string literals cannot.
    // Memory and Allocation
    // With the String type, we need memory to hold content that can change during runtime:
    // Memory must be requested from the operating system at runtime.
    // We need a way to return this memory to the operating system when we're done with our String.
    // In Rust, the memory is automatically returned once the variable that owns it goes out of scope:
    // When a variable goes out of scope, Rust calls a special function called drop automatically, and it's where the author of String can put the code to return the memory.
}
