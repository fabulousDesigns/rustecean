// Data types
// Scalar types -  int, floating point(decimal), booleans, characters
fn main() {
    // Signed integers (can be negative)
    let negative: i8 = -120;    // -128 to 127
    let normal: i32 = -200000;  // Most common, default choice
    // Unsigned integers (only positive)
    let positive: u8 = 255;     // 0 to 255
    let bigger: u32 = 200000;   // 0 to 4,294,967,295
    let decimal:f32  = 67.78999;

    let x = 2.0;
    let y: f32 = 3.0;

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * sum;
    // division
    let quotient = 95.5 / 4.3;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation
    // char
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    // compound types
    // 1. tuple - collection of csv in a parenthesis that don't have to be of the same type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring
    let (x, y, z) = tup;
    println!("{} {} {} {}", x, y, z, heart_eyed_cat);
    // accessing tuple values with index
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{} {} {}", five_hundred, six_point_four, one);
    // Arrays
    let a = [1, 2, 3, 4, 5];
    another_function()
}

fn another_function() {
    println!("Another function.");
}