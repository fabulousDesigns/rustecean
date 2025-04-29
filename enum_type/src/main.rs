enum IpAddrKind{
    V4(u8, u8, u8, u8), // You can store different types and amounts of data in each variant:
    V6(String), // A string
}
// Complex Enum Examples
// Enums can contain a wide variety of types, including structs:
enum Message {
    Quit,                   // No data
    Move {x:i32, y:i32},    // Anonymous Struct
    Write(String),          // String
    ChangeColor(i32, i32, i32), // Three i32 values
}
// Methods on Enums
// Like structs, enums can have methods defined with impl:
impl Message {
    fn call(&self) {
        // Method body
        println!("Calling call");
    }
}
// The Option Enum
// Rust doesn't have null values, but it has an enum that can express the concept of a value being present or absent:
enum Option<T>{
    Some(T),
    None,
}
fn main() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loop_back = IpAddrKind::V6(String::from("::1"));
    let m = Message::Write(String::from("hello"));
    m.call();
    let some_number = Some(5);
    let some_string = Some("a string");
    // let absent_number: Option<i32> = None
}
