use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("guess the number hoe");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("enter a number");
                continue;
            }
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
