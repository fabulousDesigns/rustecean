// 🧾 Without Nested Paths (Verbose)
use std::cmp::Ordering;
use std::io;
// ✅ With Nested Paths (Clean)
use std::{cmp::Ordering, io};
// This brings both std::cmp::Ordering and std::io into scope concisely.
// 🧾 Another Example
// ❌ Long Form
use std::io;
use std::io::Write;
// ✅ Cleaner with Nested Paths
use std::io::{self, Write};
// self means: also bring in the module itself (std::io).
// Write refers to the Write trait inside std::io.
// ⚠️ Gotcha: Only works when the base path is the same
// You cannot do this:
// ❌ INVALID: different base paths
use std::{cmp::Ordering, fs::File};
// But this is valid:
use std::cmp::{Ordering, PartialOrd};
// 🔁 Recap Memory Hack
// Think of it like:
// "std:: go into your drawer and get cmp::Ordering and io at once."
// It’s cleaner, reduces repetition, and is especially useful when importing multiple items from the same module.
fn main() {
    println!("Hello, world!");
}
