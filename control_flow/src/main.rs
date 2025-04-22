fn main() {
    let number = 6;

   // if number % 4 == 0 {
   //     println!("number is divisible by 4");
   // }else if number % 3 == 0 {
   //     println!("number is divisible by 3");
   // }else if number % 2 == 0 {
   //     println!("number is divisible by 2");
   // } else {
   //     println!("number is not divisible by 4, 3, or 2");
   // }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}