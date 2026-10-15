use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100); // inclusive

    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your Guess!");

        let mut guess = String::new(); // zwraca nowy instance String

        io::stdin()
            .read_line(&mut guess) // append to String
            .expect("Failed to read line"); // read_line zwraca Result, dlatego dostepny
        // jest .expect -> Ok/Err

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // ok <=> value
            Err(_) => continue, // Err <=> error message
        };
        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // shadowing
        // trim usuwa \n i \r\n przy usuwaniu kliknieciu enter

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            // arms make sure you handle them all
            Ordering::Less => println!("Too small"), // secret_number > guess
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// crates.io - pakiety do rust
// cargo doc --open
