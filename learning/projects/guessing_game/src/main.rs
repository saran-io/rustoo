// this is the guessing game for rustlang


//get the input output library into the game
use std::io;

//create the main function

fn main() {
    
    println!("Guess the number!");
    println!("Please input your guess.");

    //line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew! That’s a mouthful.
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}",guess);

}
