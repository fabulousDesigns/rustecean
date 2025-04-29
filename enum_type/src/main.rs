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
fn main() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loop_back = IpAddrKind::V6(String::from("::1"));
}
