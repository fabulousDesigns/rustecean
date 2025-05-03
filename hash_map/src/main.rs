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
    // we handle the Option by calling copied()
    // to get an Option<i32> rather than an Option<&i32>,
    // then use unwrap_or(0) to set score to zero if the key doesn't exist in the map. 
    // You can iterate through all key-value pairs using a for loop:
    for (key, value) in &scores {
        println!("{}: {}", key, value); // The output will be each pair
        // printed in an arbitrary order
        // (hash maps don't guarantee any specific order)
    }
    // Hash Maps and Ownership
    // For types that implement the Copy trait (like i32), 
    // values are copied into the hash map. 
    // For owned values like String, 
    // the values will be moved and the hash map will become the owner of those values.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Field_name and field_value are invalid at this point!
    // If you insert references into a hash map, the values won't be moved. 
    // However, 
    // the values that the references point to must be valid for at least as long 
    // as the hash map is valid.
    // Updating a Hash Map
    // Although hash maps can grow to contain more key-value pairs, 
    // each unique key can only have one value associated with it at a time.
    // Rust-lang Here are the various ways to handle updates:
    // 1. Overwriting a Value,
    // If you insert a key-value pair and then insert the same key with a different value, the original value will be replaced:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);
    println!("{:?}", scores);
    // 2. Adding a Key-Value Only If the Key Isn't Present, 
    // Hash maps have a special API called entry for checking 
    // whether a key exists and inserting a value if it doesn't. 
    // The entry method returns an enum called Entry 
    // that represents a value that might or might not exist.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // Insert "Yellow": 50 if "Yellow" doesn't exist (it doesn't)
    scores.entry(String::from("Yellow")).or_insert(50);
    // Try to insert "Blue": 50, but "Blue" already exists with value 10
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores); // Will print {"Yellow": 50, "Blue": 10}
}
