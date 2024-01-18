//! Guessing Game

/// # Processing a Guess:
/// 1. The first part of the guessing game program will ask for user input,
///    process that input, and check that the input is in the expected form.
use std::io;
fn main() {
    println!("Welcome to the Guessing Game!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
