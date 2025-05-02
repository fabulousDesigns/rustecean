// 🔑 Core Concept: use Brings a Path into Scope
// Think of use as creating a shortcut or alias for a longer module path — just like adding a symlink or shortcut on your desktop to a deep folder. Instead of writing the full path every time, you use a shorter name within a specific scope.
mod front_of_house {
   pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }
    }
}
use crate::front_of_house::hosting; // Bring module `hosting` into scope
// 🚫 Anti-pattern: Bringing the Function Itself
use crate::front_of_house::hosting::add_to_waitlist;
pub fn eat_at_restaurant() {
    add_to_waitlist(); // But where is it from? 🤷
    // 🧠 Why it's unidiomatic for functions? It hides context. Readers can’t tell the origin of add_to_waitlist.
}
fn main() {
    hosting::add_to_waitlist(); // Clear and Idiomatic
    // // 🧠 Why idiomatic? Because hosting::add_to_waitlist() makes it clear that the function lives in the hosting module — this improves readability.
}
