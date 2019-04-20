extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;
//use core::result::Result;
//extern crate core;
//extern crate collections;

fn main() {
    println!("Welcome to Number Guessing Game!");

    let mut guess = String::new();
    let mut secret_number = rand::thread_rng().gen_range(0, 10);

    loop {
        println!("Enter a number now. (Type exit to quit.)");

        guess.clear();

        io::stdin().read_line(&mut guess)
            .expect("Unable to read line!");

        if guess.trim().eq("exit") {
            break;
        }

        let guess_number : usize;

        match guess.trim().parse() {
            Ok(number) => {
                guess_number = number;
                println!("number {}", guess_number);
            }
            Err(error) => {
                println!("Try again. {}", error);
                continue
            }
        }

        println!("You guessed {}", guess_number);

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Smaller :-(" ),
            Ordering::Equal => { println!("Correct! :-)" );
            secret_number = rand::thread_rng().gen_range(0, 10);},
            Ordering::Greater => println!("Larger :-(" )
        }
    }
}
