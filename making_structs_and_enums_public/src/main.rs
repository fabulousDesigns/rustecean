// 🧱 STRUCTS: Public Type, Private By Default Fields
pub struct Breakfast {
    pub toast: String,         // Accessible
    seasonal_fruit: String,    // Still private
}
// Breakfast is public (can be used outside the module)
// But its fields are not public by default
// You must use pub on each field individually to expose them
fn main() {
    println!("Hello, world!");
}
