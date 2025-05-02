mod kitchen {
    #[derive(Debug)]
    pub struct Meal {
        pub name: String,
        calories: i32, // private
    }

    impl Meal {
        pub fn new(name: &str) -> Meal {
            Meal {
                name: name.to_string(),
                calories: 800, // controlled internally
            }
        }
    }

    #[derive(Debug)]
    pub enum OrderType {
        DineIn,
        TakeAway,
        Delivery,
    }
}

fn main() {
    let mut meal = kitchen::Meal::new("Burger");
    meal.name = String::from("Veggie Wrap");
    // meal.calories = 599; // ❌ won't compile

    let order = kitchen::OrderType::Delivery;
    println!("{:?}, {:?}", meal, order);
}
