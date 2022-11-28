use rand::Rng;
use std::cmp::Ordering;

use crate::command_line;

pub fn run(total_of_rounds: u32, max_rounds: u32) {
    if total_of_rounds > max_rounds {
        println!(
            "[ERROR]: o número de rounds informado é maior que o limite, {}",
            max_rounds
        );
        command_line::exit(1);
    } else if total_of_rounds == 0 {
        println!("[ERROR]: o número de rounds não pode ser 0");
        command_line::exit(1);
    }

    println!(
        "[INFO]: {} {}\n",
        total_of_rounds,
        if total_of_rounds == 1 {
            "round"
        } else {
            "rounds"
        }
    );

    let mut round: u32 = 0;

    let success = 'main_loop: loop {
        print!("[{:0>2}] ", round + 1);

        let random_number: u8 = rand::thread_rng().gen_range(0..=255);
        let second_random_number: u8 = rand::thread_rng().gen_range(0..=255);

        match random_number.cmp(&second_random_number) {
            Ordering::Less => println!("{:0>3} < {:0>3}", random_number, second_random_number),
            Ordering::Greater => println!("{:0>3} > {:0>3}", random_number, second_random_number),
            Ordering::Equal => {
                println!("{:0>3} = {:0>3}", random_number, second_random_number);

                break 'main_loop false;
            }
        }

        if round == total_of_rounds - 1 {
            break 'main_loop true;
        }

        round += 1;
    };

    println!("");

    if success {
        println!("Vitória!!");
    } else {
        println!("Perdemos no round {:0>3}...", round + 1);
    }
}
