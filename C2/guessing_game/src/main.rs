use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess");

        // variables are immutable by default
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // references are immutable by default too I guess??
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // note could also do ".. {}", guess
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!") ;
                break;
            },
        }
    }
}
