use rand::Rng;
use std::{cmp::Ordering, io, process::Command};

fn main() {
    println!("GUESSING GAME");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("seu chute: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Informe um valor válido");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, informe um número válido");
                continue;
            }
        };

        Command::new("clear").status().unwrap();

        println!("você chutou: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo"),
            Ordering::Equal => {
                println!("Parabéns, você acertou");
                break;
            }
            Ordering::Greater => println!("Muito alto"),
        }
    }
}
