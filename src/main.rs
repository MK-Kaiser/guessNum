// Author: Mark Kaiser
// Date: 04/04/2020
// Description: My first rust program, a basic number guessing game.
use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    let secret_num = rand::thread_rng().gen_range(1, 101);
    
    loop {
        println!("Guess a number from 1 to 100!");

        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("That is correct, You win!");
                break;
            }

        }    
    }
}