mod loops_returning_values;
mod loops_label;
mod while_loop;
mod for_loop;
mod nested_if;

pub enum Program {
    NestedIf,
    LoopsReturningValues,
    LoopsLabel,
    WhileLoop,
    ForLoop
}

const PROGRAM_RUNNING: Program = Program::ForLoop;

fn main() {
    match PROGRAM_RUNNING {
        Program::NestedIf => nested_if::new(),
        Program::LoopsReturningValues => loops_returning_values::new(),
        Program::LoopsLabel => loops_label::new(),
        Program::WhileLoop => while_loop::new(),
        Program::ForLoop => for_loop::new(),
    }
}
