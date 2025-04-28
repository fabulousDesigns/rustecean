// Define a struct
struct Person{
    name: String,
    age: u8,
}
// Implement methods on the Person struct
impl Person {
    // Method that borrows self immutably (read-only)
    fn introduce(&self) -> String {
        format!("Hi, I'm {} and I'm {} years old.", self.name, self.age)
    }

    fn have_birthday(&mut self) {
        self.age += 1;
        println!("Happy birthday! {} is now {} years old", self.name, self.age);
    }
    // associated function constructors
    fn new(name: String, age: u8) -> Person {
        Person {
            name,
            age,
        }
    }
}
// Rectangle struct
struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle {
    // Method that calculates area
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }
    // Method that takes another rectangle as a parameter
    fn can_fit(self: &Self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    // Method that takes multiple parameters
    fn scale(&mut self, width_factor: u32, height_factor: u32) {
        self.width *= width_factor;
        self.height *= height_factor;
    }
    // Associated function that creates a square
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height:size
        }
    }
}
// struct Counter
struct Counter{
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
    fn increment(self: &mut Self) -> &mut Self {
        self.count += 1;
        self
    }
    fn add(&mut self, num: u32)-> &mut Self{
        self.count += num;
        self
    }
    fn reset(&mut self) -> &mut Self{
        self.count = 0;
        self
    }
    fn get_count(self: &Self) -> u32{
        self.count
    }
}
fn main() {
    // using the associated function (constructor)
    let mut bernard = Person::new("Bernard".to_string(), 25);
    // using an immutable method
    println!("{}", bernard.introduce());
    // Using a mutable method
    bernard.have_birthday();
    // See the change
    println!("{}", bernard.introduce());

    let mut rect1 = Rectangle{
        width: 10,
        height: 20,
    };
    let rect2 = Rectangle::square(30);
    println!("Rectangle 1 area: {}", rect1.area());
    println!("Rectangle 2 area: {}", rect2.area());
    println!("Can rect1 fit rect2? {}", rect1.can_fit(&rect2));
    rect1.scale(5, 2);
    println!("After scaling, rect1 dimensions: {}x{}", rect1.width, rect1.height);

    let mut counter = Counter::new();
    // Chain methods together
    counter.increment()
        .increment()
        .add(5)
        .increment();

    println!("Current count: {}", counter.get_count()); // Should be 8

    // Chain more methods
    counter.reset()
        .add(10);

    println!("After reset and add: {}", counter.get_count()); // Should be 10
}
// Creating a method that just reads data (introduce)
// Creating a method that modifies data (have_birthday)
// Creating an associated function that acts as a constructor (new)

//
// A method with no parameters beyond &self
// A method that takes another struct as a parameter
// A method that takes multiple parameters and modifies the struct
// An associated function that creates a specialized instance