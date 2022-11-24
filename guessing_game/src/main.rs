use rand::Rng;
use std::{cmp::Ordering, io, process};

fn main() {
    println!("GUESSING GAME\n");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    let mut guess: String;
    let mut attempts: u16 = 0;

    loop {
        guess = String::new();

        if attempts > 1 {
            println!("{attempts} attempts");
        }

        println!("guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("cannot read guess");

        process::Command::new("clear").status().unwrap();

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[!] invalid guess");
                continue;
            }
        };

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
