// Exercise 1: Vector Creation and Basic Operations
// Exercise 1: Complete the code to create and manipulate vectors
fn main() {
    // 1. Create an empty vector that will store integers
     let mut v: Vec<i32> = Vec::new();
    // 2. Create a vector with initial values 1, 2, 3, 4, 5 using the vec! macro
     let v2 = vec![1, 2, 3, 4, 5];
    // 3. Add elements 6, 7, and 8 to the first vector
    v.push(6);
    v.push(7);
    v.push(8);
    // 4. Print both vectors
    println!("The first vector is {:?}", v); // The first vector is [6, 7, 8]
    println!("The second vector is {:?}", v2); // The second vector is [1, 2, 3, 4, 5
    // 5. Get the 3rd element from the second vector using [] indexing
    let _third: &i32 = &v2[2];
    // 6. Attempt to get the 10th element using the get() method and handle the None case
    let tenth: &i32 = &v2[9];
    match v2.get(9) {
        Some(tenth) => println!("The tenth element is {}", tenth),
        None => println!("There is no tenth element"),
    }
    // 7. Create a mutable clone of the second vector and add 10 to each element
    let mut clone_v2 = v2.clone();
    for i in &mut clone_v2 {
        *i += 10;
    }
    // 8. Print the length and capacity of all vectors
    println!("clone_v2 is {:?}", clone_v2);
    println!("clone_v2 has length {} and capacity {}", clone_v2.len(), clone_v2.capacity());
    println!("v2 has length {} and capacity {}", v2.len(), v2.capacity());
    println!("v has length {} and capacity {}", v.len(), v.capacity());
}
