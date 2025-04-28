struct Rectangle { // We define a struct called Rectangle with two fields.
    width: u32,    // field 1
    height: u32,   // field 2
}
impl Rectangle {   // Define methods associated with the Rectangle struct.
    fn area(&self) -> u32 {  // Defines a method called area that: Takes &self as its first parameter (borrowing the struct instance) and Returns a u32 value
        self.width * self.height
    }
    // Methods with Additional Parameters
    // Methods can take additional parameters beyond self:
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    let sq = Rectangle::square(30);
    println!("The area of the square is {} square pixels.", sq.area());
}

// The self Parameter
// The self parameter is special in Rust methods:
// &self is short for self: &Self (borrows the instance immutably)
// &mut self is short for self: &mut Self (borrows the instance mutably)
// self is short for self: Self (takes ownership of the instance)
// Most methods use &self because they only need to read data, not modify it or take ownership.

