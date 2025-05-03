// What is a String in Rust?
// Rust has two main string types:
//
// The string slice str (usually seen in its borrowed form &str). This is a fundamental type in the core language. String literals (e.g., "hello") are stored in the program's binary and are therefore string slices. Rust-lang
// The String type, which is provided by Rust's standard library rather than coded into the core language. It is a growable, mutable, owned, UTF-8 encoded string type. Rust-lang
//
// This dual nature of strings in Rust is important to understand. When people refer to "strings" in Rust, they could mean either String or &str.
// String Fundamentals
// The String type is actually implemented as a wrapper around a vector of bytes (Vec<u8>) with some extra guarantees, restrictions, and capabilities. This means many operations available with Vec<T> are also available with String. Rust-lang
// Crucially, both String and string slices are UTF-8 encoded. This means they can represent any valid UTF-8 character from any language. Rust-lang
fn main() {
    // using string::new()
    let mut s = String::new(); // a new, empty string that content can be added later to
    // using to_string()
    let data = "initial contents";
    let s = data.to_string();
    // works directly on literals too
    let s = "initial contents".to_string();
    // using String::from()
    let s = String::from("initial contents"); // Both String::from() and to_string() do the same thing, so which you choose is a matter of style and readability.
    // UTF-8 Support
    // Because strings are UTF-8 encoded, we can include any properly encoded data in them. This means we can represent text in any language:
    let hello = String::from("<UNK> <UNK>"); //
    let hello = String::from("Hello");           // English
    let hello = String::from("Hola");            // Spanish
    let hello = String::from("こんにちは");        // Japanese
    let hello = String::from("안녕하세요");        // Korean
    let hello = String::from("你好");            // Chinese
    let hello = String::from("Здравствуйте");    // Russian
    let hello = String::from("नमस्ते");           // Hindi
    // Updating a String
    // A String can grow in size and its contents can change, similar to a Vec<T>.
    // Appending with push_str and push
    let mut s = String::from("foo");
    s.push_str("bar"); // s contains "foobar"
    // push_str doesn't take ownership
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    // adding a single chararcter with push
    let mut s = String::from("lo");
    s.push('l'); // s now contains "lol"
    // Concatenation with the + Operator
    // You can combine strings using the + operator: Rust-lang
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // Note: s1 has been moved here and can no longer be used
    // Using the format! Macro
    // A more readable alternative for combining multiple strings is the format! macro: Rust-lang
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");  // s becomes "tic-tac-toe"

    let namaste = String::from("नमस्ते"); // If we look at the raw bytes, it's stored as:
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // As Unicode scalar values (Rust's char type), it looks like:
    // ['न', 'म', 'स', '्', 'त', 'े'] Rust-lang
    // But as grapheme clusters (what we'd call "letters"), it's:
    // ["न", "म", "स्", "ते"] Rust-lang
    // This demonstrates why simply returning a "character" at an index is ambiguous in UTF-8.
    // String Slicing
    // You can create slices of strings using range syntax, but you have to be careful:
    let hello = "Здравствуйте";
    let s = &hello[0..4];  // s will be "Зд" (the first two Cyrillic characters)
    // This works because each Cyrillic character takes 2 bytes in UTF-8. But if you try to slice in the middle of a character (e.g., &hello[0..1]), Rust will panic at runtime because byte index 1 is not a character boundary. Rust-lang
    // Use extreme caution with string slicing to avoid runtime panics.
    // Iterating Over Strings
    // Instead of indexing, Rust provides safer methods to work with string components:
    // Iterating by Characters:
    for c in "Зд".chars() {
        println!("{c}");
    }
    // Output:
    // З
    // д
    // iterating by bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // Output:
    // 208
    // 151
    // 208
    // 180
    // Iterating by grapheme clusters (complete "letters" as we'd recognize them) is more complex and requires external crates from crates.io. Rust-lang
    // String Methods
    // The standard library provides many useful methods for working with strings:
    //
    // contains() - Check if a string contains a pattern
    // replace() - Replace occurrences of a pattern
    // split() - Split a string by a pattern
    // trim() - Remove whitespace
    // Many more in the standard library documentation
    //
    // Why Strings Are Complex in Rust
    // Strings are inherently complicated, and different programming languages make different choices about how to present this complexity. Rust has chosen to make correct handling of Unicode data the default behavior, which exposes more complexity upfront but prevents subtle bugs later. Rust-lang
    // This design philosophy matches Rust's overall approach: making potential problems explicit rather than hiding them.
}
