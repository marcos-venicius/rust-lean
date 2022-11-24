use rand::Rng;
use std::{cmp::Ordering, io, process};

fn main() {
    println!("GUESSING GAME\n");

    // generate a random number between 0 and 100
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    let mut attempts: u16 = 0;

    loop {
        let mut guess = String::new();

        if attempts > 0 {
            println!("[{attempts}] attempts");
        }

        println!("your guess: ");

        // get user niput
        io::stdin()
            .read_line(&mut guess)
            .expect("cannot read guess");

        // clear output
        process::Command::new("clear").status().unwrap();

        // parse the guess string text to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please inform a valid number");
                continue;
            }
        };

        // compare values
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
            Ordering::Greater => println!("Too big!!"),
        }

        attempts += 1;
    }
}
