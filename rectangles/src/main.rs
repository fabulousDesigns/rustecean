// Refactoring with struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "This area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    let rectangle1 = (30, 50);
    println!(
        "This area of the rectangle is {} square pixels.",
        area_refactoring_with_struct_tuple(rectangle1)
    );
    let rectangle2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rectangle2: {:#?}", rectangle2);
    println!("The area of the rectangle is {} square pixels", area_refactoring_with_struct(&rectangle2))
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn area_refactoring_with_struct_tuple(rectangle: (u32, u32)) -> u32 {
    let (width, height) = rectangle;
    width * height
}
fn area_refactoring_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}