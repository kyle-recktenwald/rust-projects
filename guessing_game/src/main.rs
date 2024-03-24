use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        /*
         * Examples of shadowing, trimming whitespace, converting 
         * a String to an unsigned 32 bit integer, and handling an error:
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, //Variant
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        /*
         * A match expresssion is made up of arms, which consist of a
         *  pattern to match against and the code that should run
         *  if the value fits the pattern
         * Ordering::Less is an example of a variant
         */
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
