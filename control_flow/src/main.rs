fn divisible(){
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    }else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    }else{
        println!("number is not divisible by 4, 3, or 2");
    }
}
// Repeating Code with loop
// The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
// As an example, change the src/main.rs file in your loops directory to look like this:
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    // nested loops
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
    }
    // loop {
    //     println!("The value of number is: {number}");
    // }
    // println!("The value of number is: {number}");
    // divisible();
