fn main() {
    // creating a vector
    // 1. using Vec::new()
    let v: Vec<i32> = Vec::new();
    // 2. Using the vec! macro
    let v = vec![1, 2, 3];
    // modifying a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // accessing elements in a vector
    // 2 main ways
    // 1. using index syntax with []
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2]; // get the third element
    println!("The third element is {}", third);
    // 2. using the get method
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
    // The key difference between these two methods is how they handle invalid indices:
    // When using the [] method with an index that doesn't exist (like trying to access index 100 in a 5-element vector), the program will panic. This is best used when you want your program to crash if there's an attempt to access an element past the end of the vector. Rust-lang
    // When the get method is passed an index that is outside the vector, it returns None without panicking. You would use this method if accessing an element beyond the range of the vector might happen occasionally under normal circumstances. Mit
    // iterating over vector Elements
    // immutable iteration
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    // mutable iteration
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator.
    }
    // Dropping a Vector
    // Like any other struct, a vector is freed when it goes out of scope. When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up
    // Storing Different Types in a Vector
    // Vectors can only store values that are of the same type. This can be inconvenient when you need to store a list of items of different types. Rust-lang
    // To work around this limitation, you can use an enum to define different types under a single type:
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled. Typeerror
    // If you don't know all the possible types at compile time, you can use trait objects, which will be covered in later chapters.
    // Important Vector Concepts to Remember
    // Memory Layout: Vectors store values next to each other in memory. Adding a new element onto the end of the vector might require allocating new memory and copying the old elements if there isn't enough room. This is why borrowing rules prevent holding references to elements while adding new ones. Rust-lang
    // Capacity and Growth: A vector is represented using 3 parameters, including capacity, which indicates how much memory is reserved for the vector. The vector can grow as long as the length is smaller than the capacity. When this threshold needs to be surpassed, the vector is reallocated with a larger capacity. Rust-lang
    // Performance Considerations: When a vector needs to reallocate, it typically doubles its capacity to amortize the cost of reallocation over many push operations.
}
