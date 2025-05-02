mod back_of_the_house{
    // 🧱 STRUCTS: Public Type, Private By Default Fields
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,         // Accessible
        seasonal_fruit: String,    // Still private
    }
    // Breakfast is public (can be used outside the module), 
    // But its fields are not public by default. 
    // You must use pub on each field individually to expose them
    // Why?
    // This design is intentional:
    // Structs often contain internal logic or sensitive data you want to hide. 
    // Rust encourages you to control how structs are created/ modified. 
    // You typically expose a constructor method like:
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // This pattern:
    // Lets external code choose toast
    // But hides seasonal_fruit from tampering
    // This is textbook encapsulation.
    // 🧁 ENUMS: Public Everything by Default
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
    // When you say pub enum, all variants are automatically public
    // You don’t need to write pub on each variant — that's different from structs
    // Why?
    // Because:
    // Enums are pattern matched,
    // It wouldn’t make sense to have private variants — you couldn’t match on them!
    // Their power comes from variant access, so Rust exposes them all at once
}
fn main() {
    let soup_order = back_of_the_house::Appetizer::Soup;
    let salad_order = back_of_the_house::Appetizer::Salad;
    println!("{:?}, {:?}", soup_order, salad_order);
}
