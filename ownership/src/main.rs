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
    // Move Semantics
    // Multiple variables can interact with the same data in different ways. Let's look at an example with an integer:
    let x = 5;
    let y = x;
    // Here, we bind the value 5 to x, then make a copy of that value and bind it to y. Now we have two variables, both equal to 5. This works because integers are simple values with a known, fixed size, and these values are pushed onto the stack.
    // But what happens with the String type?
    let s1 = String::from("hello");
    let s2 = s1;
    // Clone
    // If we actually do want to deeply copy the heap data of a String and not just the stack data, we can use a common method called clone:
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);
    // When you see a call to clone, you know some arbitrary code is being executed and that code may be expensive. It's a visual indicator that something different is going on.
    // Stack-Only Data: Copy
    // Now let's look at this code with integers:
    let w = 5;
    let z = w;
    // This code seems to contradict what we just learned: we don't have a call to clone, but x is still valid after assigning to y.
    // The reason is that types like integers have a known size at compile time and are stored entirely on the stack, so copies of the actual values are quick to make. There's no difference between deep and shallow copying here, so calling clone wouldn't do anything different from the usual shallow copy.
    // All the integer types
    // The Boolean type
    // All the floating-point types
    // The character type char
    // Tuples, but only if they contain types that also implement Copy
    //Ownership and Functions
    // The semantics for passing a value to a function are similar to assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.
    let greetings = String::from("hello");
    takes_ownership(greetings);
    // println!("{}", greetings); // can't use a value after it has been moved
    let test_makes_copy = 5;
    makes_copy(test_makes_copy);
    println!("{test_makes_copy}"); // Nothing happens because Rust will allow copy
    // Return Values and Scope
    // Returning values can also transfer ownership. Here's an example:
    let s4 = gives_ownership();         // gives_ownership moves its return
    // value into s1
    let s5 = String::from("hello");     // s2 comes into scope
    let s6 = takes_and_gives_back(s2);  // s2 is moved into
    let mut b_test = String::from("hello");
    change(&mut b_test);
    // Mutable References
    // To fix the previous code, we need to use a mutable reference:
    // The slice type
    // The final section in the ownership chapter introduces slices, which are references to a contiguous sequence of elements in a collection rather than the whole collection.
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into the scope
    a_string // a_string is returned and moves out to the calling function
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}