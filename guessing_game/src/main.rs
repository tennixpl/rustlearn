use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    // `guess` needs to be mutable so later we can get the value of stdin
    // and put into guess for use later.

    io::stdin()
        .read_line(&mut guess) // referencing the guess variable but why &mut, needs to be mut to change guess, and the & makes it refer to orignal guess pointer Chapter 4 
        .expect("Failed to read line"); // error handling

    println!("You guessed: {guess}");
}