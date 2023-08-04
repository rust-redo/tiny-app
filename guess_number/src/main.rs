use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("> Guess the number between 1-100!");

    /* generate a random number between 1-100 */
    let secret_number = rand::thread_rng().gen_range(1..=100);

    /*  loop doesn't exit until process receives SIGINT or user inputs the right number */
    loop {
        println!("> Please input your guess:");

        let mut guess = String::new();

        /* wait for the user input */
        io::stdin()
            .read_line(&mut guess)
            .expect("> Failed to read line");

        /* 
        * covert the input string to a integer number,
        * output the error info if the input string is invalid
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Err: {}", err);
                continue;
            },
        };

        println!("> You guessed: {guess}");

        /*
        * compare the input number with the random number,
        * exit loop when they are equal
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("> Too small!"),
            Ordering::Greater => println!("> Too big!"),
            Ordering::Equal => {
                println!("> You win!");
                break;
            }
        }
    }
}