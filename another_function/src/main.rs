fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
fn main() {
    let y = {
        let x = 1;
        x + 1
    };
    {
        let x = 1;
        let y = 1;
        let z = x + y;
    }
    let x = five();
    let x = plus_one(x);
    println!("{x}");
    println!("The value of x is: {x}");
    // println!("The value of y is: {}", y);
}