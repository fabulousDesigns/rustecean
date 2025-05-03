// Introduction to Hash Maps
// The HashMap<K, V> type in Rust stores a mapping of keys of type K to values of type V using a hashing function. In other programming languages, this data structure might be called a hash, map, object, hash table, dictionary, or associative array.
// Creating a New Hash Map
// Unlike vectors and strings, hash maps are not automatically included in the prelude, so you need to import them:
use std::collections::HashMap;
fn main() {
    // You can create an empty hash map and add elements with insert:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // Like vectors, hash maps store their data on the heap.
    // This HashMap has keys of type String and values of type i32.
    // Also, like vectors, hash maps are homogeneous:
    // all keys must have the same type, and all values must have the same type.
    // Accessing Values in a Hash Map
    // To get a value from a hash map, use the get method with the key:
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("team {} scores: {}", team_name, score);
    // The get method returns an Option<&V>.
    // If there's no value for that key, it returns None.
    // In the example above,
    // we handle the Option by calling copied() to get an Option<i32> rather than an Option<&i32>,
    // then use unwrap_or(0) to set score to zero if the key doesn't exist in the map. 
    // You can iterate through all key-value pairs using a for loop:
    for (key, value) in &scores {
        println!("{}: {}", key, value); // The output will be each pair
        // printed in an arbitrary order
        // (hash maps don't guarantee any specific order)
    }
}
