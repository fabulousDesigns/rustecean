// Option addresses a fundamental challenge in Rust: representing values that might be missing
// The Problem with Null
// First, the text explains the issues with null in other languages:
// Null is a value that means "no value" (a contradiction in itself)
// Tony Hoare (who invented null) called it his "billion-dollar mistake"
// The main problem: null reference errors are extremely common and easy to make
// In languages with null, variables always exist in one of two states: null or not-null
// Rust's Alternative: Option<T>
// Instead of null, Rust uses an enum to explicitly represent this concept:
enum Option<T>{
    None,     // absence of a value
    Some(T),  // presence of a value of type T
}
// Key insights about Option<T>:
// It's so fundamental that it's included in the prelude (no need to import)
// The variants Some and None can be used directly without Option:: prefix
// <T> is a generic type parameter (full details in Chapter 10)
// The Some variant can hold one piece of data of any type
// Each different type used for T creates a different overall type
fn main() {
    let some_number = Some(5);          // Type is Option<i32>
    let some_char = Some('e');         // Type is Option<char>
    let absent_number: Option<i32> = None;          // Type annotation required for None

    // This code doesn't compile!
    let x: i8 = 5;
    let y: Option<i32> = Some(5);
    let sum = x + y;  // Error! Can't add i8 and Option<i8>
}
