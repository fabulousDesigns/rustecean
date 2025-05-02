// The glob operator (*) allows you to bring everything that's public in a module into scope at once—without having to list each item individually.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
}
// ✅ With Glob:
use crate::front_of_house::hosting::*; // This imports all public items from hosting into the current scope—so you can call them directly.
// ⚠️ Be Careful: Glob Can Be Ambiguous
// Glob imports are less explicit and can sometimes lead to name conflicts or unclear origins. For example:
use std::io::*;
use std::fs::*;
// If both io and fs contain a function named read, now you won’t know which one you’re calling.
// That’s why glob imports are often discouraged in general-purpose code unless:

// ✅ When It Is Idiomatic
// In tests, when pulling in a lot of helper functions:
use my_crate::test_helpers::*;
// When using prelude modules that are explicitly designed to be glob-imported (e.g., std::prelude::v1::* is auto-imported by default).
// 🧠 Memory Trick
// Think of * like a wildcard "grab everything off the shelf"—helpful in a pinch, messy if you're not careful.
fn main() {
    add_to_waitlist();
    seat_at_table();
}

