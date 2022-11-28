mod loops_returning_values;
mod loops_label;
mod nested_if;

pub enum Program {
    NestedIf,
    LoopsReturningValues,
    LoopsLabel
}

const PROGRAM_RUNNING: Program = Program::LoopsLabel;

fn main() {
    match PROGRAM_RUNNING {
        Program::NestedIf => nested_if::new(),
        Program::LoopsReturningValues => loops_returning_values::new(),
        Program::LoopsLabel => loops_label::new(),
    }
}
