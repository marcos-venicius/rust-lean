use std::io;
use std::process;

pub fn exit(code: i32) {
    process::exit(code);
}

pub fn wait_user_press_enter() {
    let _ = io::stdin().read_line(&mut String::new());
}

pub fn read_number_from_stdin() -> u32 {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(size) => size,
        Err(_) => {
            println!("[ERROR]: não conseguimos ler a entrada");
            process::exit(1);
        }
    };

    let parsed_input: u32 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("[ERROR]: o valor informado não é um número");
            process::exit(1);
        }
    };

    parsed_input
}

#[cfg(windows)]
pub fn clear_output() {
    process::Command::new("cls").status().unwrap();
}

#[cfg(not(windows))]
pub fn clear_output() {
    process::Command::new("clear").status().unwrap();
}
