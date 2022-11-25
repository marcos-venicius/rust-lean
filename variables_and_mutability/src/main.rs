mod constants;
mod shadowing;
mod immutable_variables;

const MODULE: &str = "shadowing";

fn main() {
    match MODULE {
        "immutable_variables" => immutable_variables::new(),
        "constants" => constants::new(),
        "shadowing" => shadowing::new(),
        _ => todo!(),
    }
}
