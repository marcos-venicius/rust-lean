mod loops;
mod nested_if;

pub enum Program {
    NestedIf,
    Loops,
}

const PROGRAM_RUNNING: Program = Program::Loops;

fn main() {
    match PROGRAM_RUNNING {
        Program::NestedIf => nested_if::new(),
        Program::Loops => loops::new(),
    }
}
