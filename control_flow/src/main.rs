mod loops_returning_values;
mod nested_if;

pub enum Program {
    NestedIf,
    LoopsReturningValues,
}

const PROGRAM_RUNNING: Program = Program::LoopsReturningValues;

fn main() {
    match PROGRAM_RUNNING {
        Program::NestedIf => nested_if::new(),
        Program::LoopsReturningValues => loops_returning_values::new(),
    }
}
