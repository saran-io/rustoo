// this is the guessing game for rustlang
// this is part of the rustlang book and learning coding

//get the input output library into the game
//get the rand librariy into the game
use std::io;
use rand::Rng;
use std::cmp::Ordering;


//create the main function

fn main() {
    
    println!("Guess the number!");

    println!("Create a secret number from the 1-100 range");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");

    //create a loop for mulitple values 

    loop {
    println!("Please input your guess.");

    //line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew! That’s a mouthful.
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {}",guess);

    //compare the guesses number with the secret number
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
    }
}
