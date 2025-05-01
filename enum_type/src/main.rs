// Enums allow you to define a custom type by listing all its possible variants
// Enumerate - list items one by one,so an enum is literally listing all possible values of a type
// Enums in Rust are more powerful than enums in many other languages (as we'll see)
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}
// Breaking this down:
//
// enum keyword declares we're creating an enumeration
// IpAddrKind is the name of our new custom type
// Inside the braces are the "variants" (possible values): V4 and V6
// Each variant is separated by a comma
// By convention, variant names are PascalCase (like V4, V6)
//
// At this point, our enum just defines the possible variants but doesn't store any additional data. We've created a new type called IpAddrKind that we can use throughout our code.
fn main() {
    println!("{:?}", IpAddrKind::V4);
    println!("{:?}", IpAddrKind::V6);
}