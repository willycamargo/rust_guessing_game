use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("{}", "Guess the number!".bright_purple());
    println!("Please input your guess.");
    
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            },
        };
        
        println!("You guessed {guess}!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".bright_red()),
            Ordering::Greater => println!("{}", "Too big!".bright_red()),
            Ordering::Equal => {
                println!("{}", "You win!".bright_green());
                break;
            },
        }

        println!("Try again!")
    }
}
