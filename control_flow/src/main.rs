mod nested_if;

pub enum Program {
    NestedIf,
}

const PROGRAM_RUNNING: Program = Program::NestedIf;

fn main() {
    match PROGRAM_RUNNING {
        Program::NestedIf => nested_if::new(),
    }
}
