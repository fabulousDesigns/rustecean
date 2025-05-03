// Exercise 2: Implement the following functions for vector processing
fn main() {
    let vec1 = vec![10, 20, 30, 40, 50];
    let vec2 = vec![3, 6, 9, 12, 15];

    // Print the sum of all elements in vec1
    println!("Sum of vec1: {}", sum_elements(&vec1));

    // Create a new vector containing only even numbers from vec1
    let even_only = filter_even(vec1);
    println!("Even numbers: {:?}", even_only);

    // Create a new vector by multiplying each element of vec2 by 2
    let doubled = double_elements(&vec2);
    println!("Doubled vec2: {:?}", doubled);

    // Create a new vector by combining elements from vec2, and even_only
    // The result should have all even_only elements followed by all vec2 elements
    let combined = combine_vectors(&even_only, &vec2);
    println!("Combined vector: {:?}", combined);
}

// Implement these functions:
fn sum_elements(vec: &Vec<i32>) -> i32 {
    // Return the sum of all elements in the vector
    let mut sum = 0;
    for i in vec{
        sum += *i;
    }
    sum
}

fn filter_even(vec: Vec<i32>) -> Vec<i32> {
    // Return a new vector containing only even numbers
    let mut evens = Vec::new();
    for i in vec{
        if i % 2 == 0 {
            evens.push(i);
        }
    }
    evens
}

fn double_elements(vec: &Vec<i32>) -> Vec<i32> {
    // Return a new vector with each element doubled
    let mut doubles = Vec::new();
    for i in vec{
        doubles.push(*i * 2);
    }
    doubles
}

fn combine_vectors(vec1: &Vec<i32>, vec2: &Vec<i32>) -> Vec<i32> {
    // Return a new vector combining elements from both vectors
    let mut combined = Vec::new();
    combined.extend(vec1);
    combined.extend(vec2);
    combined
}