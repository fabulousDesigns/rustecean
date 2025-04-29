enum IpAddrKind{
    V4(String),
    V6(String),
    V8(u32,u32,u32,u32)
}
fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loop_back = IpAddrKind::V6(String::from("::1"));
}
