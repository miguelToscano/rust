use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game");

    // thread_rng is how the Rng library is used, you dont need to know it first hand since its specified in the library docs
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess: ");

        // creates a new string instance
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line"); // prints an error and exits the program if an error occurs
    
        // this is called "shadowing". Allows you to "overwrite a previously existing variable with a new value", very handy when input sanitizing
        // parse method tries to convert the value to the inferred u32 type
        // match is used to properly handle error cases since parse method returns a Result type (Ok, Err) response
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid value");
                continue;
            },
        };
    
        println!("You guessed: {}", guess);
    
        // the way to compare values and handle possible cases
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win, secret number was: {}", secret_number);
                break;
            },
        }
    }
}
