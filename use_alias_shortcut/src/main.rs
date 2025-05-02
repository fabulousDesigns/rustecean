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
// 🧭 Scope Matters!
// If you use something outside a module (e.g., at the crate root),it won’t work inside a different module unless you re-use it or use a path like super::.
mod customer {
    // `hosting` isn't in scope here unless we re-import it
    pub fn eat() {
        super::front_of_house::hosting::add_to_waitlist(); // Or add `use` here
    }
}
// 🧱 Structs, Enums, and Aliases
// For types like structs/enums, it's idiomatic to bring the item itself into scope:
use std::collections::HashMap;
fn main() {
    hosting::add_to_waitlist(); // Clear and Idiomatic
    // // 🧠 Why idiomatic? Because hosting::add_to_waitlist() makes it clear that the function lives in the hosting module — this improves readability.
    let mut map = HashMap::new();
    // Why? Because the type name is capitalized and it’s visually clear it's not local.
}
// 🤯 What if Two Names Clash?
use std::fmt::Result;
use std::io::Result as IoResult; // Rename using `as`

fn f1() -> Result {
    Ok(())
}

fn f2() -> IoResult<()> {
    Ok(())
}
// 🧠 Trick: Use as to avoid name collisions in scope — especially useful in large projects with common type names like Error, Result, Config, etc.
