use std::io;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    // Generate and display the random number
    let secret_number = rand::thread_rng().gen_range(1..=100); // explain todo()
    // rand::thread_rng function that gives us the particular random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system. 
    // The gen_range method takes a range expression as an argument and generates a random number in the range.
   
    println!("The secret number is: {secret_number}");


    println!("Please input your guess.");

    let mut guess = String::new();
    // `guess` needs to be mutable so later we can get the value of stdin
    // and put into guess for use later.

    io::stdin()
        .read_line(&mut guess) // referencing the guess variable but why &mut, needs to be mut to change guess, and the & makes it refer to orignal guess pointer Chapter 4 
        .expect("Failed to read line"); // error handling, what would cause this? 

    println!("You guessed: {guess}");
}