use rand::{self, Rng};
use std::{cmp::Ordering, io};

pub fn guessing_game() {
    // choose a random number
    let answer: u32 = rand::thread_rng().gen_range(0..=100);
    // in a loop, ask user for integer input
    loop {
        println!("Guess the number");

        // stdin does not overwrite but append, that is why this definition has to be in the loop to get a new var for each guess
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&answer) {
            Ordering::Greater => println!("Guess is bigger than the answer!!!"),
            Ordering::Less => println!("Guess is smaller than the answer!!!"),
            Ordering::Equal => {
                println!("You have guessed correctly!");
                break;
            }
        }
    }
}
