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
    // Like vectors, hash maps store their data on the heap. This HashMap has keys of type String and values of type i32. Also like vectors, hash maps are homogeneous: all keys must have the same type, and all values must have the same type.
}
