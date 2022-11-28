mod game;
mod command_line;

use crate::command_line::{clear_output, read_number_from_stdin, wait_user_press_enter};

fn main() {
    loop {
        clear_output();

        println!("número de rounds: ");

        let total_of_rounds: u32 = read_number_from_stdin();

        clear_output();

        game::run(total_of_rounds, 100);

        println!("\npressione ENTER para jogar novamente ou CTRL+C para sair");

        wait_user_press_enter();
    }
}
