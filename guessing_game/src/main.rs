use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!");

    // Generate a random secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Create a mutable String to hold the user's input
    let mut guess = String::new();

    // Read the user's input from standard input

    loop {
        println!("Please enter your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // println!("You guessed: {}", guess);
    // println!("The secret number is: {}", secret_number);
}
