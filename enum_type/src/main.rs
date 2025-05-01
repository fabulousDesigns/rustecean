// Enums allow you to define a custom type by listing all its possible variants
// Enumerate - list items one by one,so an enum is literally listing all possible values of a type
// Enums in Rust are more powerful than enums in many other languages (as we'll see)
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}
// Breaking this down:
// enum keyword declares we're creating an enumeration
// IpAddrKind is the name of our new custom type
// Inside the braces are the "variants" (possible values): V4 and V6
// Each variant is separated by a comma
// By convention, variant names are PascalCase (like V4, V6)
// At this point, our enum just defines the possible variants but doesn't store any additional data. 
// We've created a new type called IpAddrKind that we can use throughout our code.
// enums in functions
fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}
// Adding data to enums
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
// This works but is verbose - we're using a struct that contains the enum.
// Approach 2
#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String)
}
impl IpAddrEnum {
    fn call(&self) {
        println!("{:?}", self);
    }
}
// approach 3
#[derive(Debug)]
enum IpAddressEnum{
    V4(u8, u8, u8, u8),
    V6(String)
}
fn main() {
    println!("{:?}", IpAddrKind::V4);
    println!("{:?}", IpAddrKind::V6);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{:?}", home.kind);
    println!("{:?}", home.address);
    let address_v4 = IpAddrEnum::V4(String::from("127.0.0.1"));
    let address_v6 = IpAddrEnum::V6(String::from("::1"));
    println!("address_v4: {:?}, address_v6: {:?}", address_v4, address_v6);
    let ip_addr_enum_v4 = IpAddressEnum::V4(127,0,0,1);
    let ip_addr_enum_v6 = IpAddressEnum::V6(String::from("::1"));
    println!("ip_addr_enum_v4: {:?}", ip_addr_enum_v4);
    println!("ip_addr_enum_v6: {:?}", ip_addr_enum_v6);
}