extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let low = 1;
    let high = 100;
    println!("Guess the number from {0} to {1}!", low, high);
    let secret_number = rand::thread_rng().gen_range(low, high+1);
//    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let msg = format!("You guessed: {}. ", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} Too small!", msg),
            Ordering::Greater => println!("{} Too big!", msg),
            Ordering::Equal => {
                println!("{} You win!", msg);
                break;
            },
        }
    }
}
