mod integer_overflow;
mod operators;
mod type_values;
mod floating;
mod tuples;

pub enum Program {
    IntegerOverflow,
    TypeValues,
    Operators,
    Floating,
    Tuples,
}


const PROGRAM: Program = Program::Tuples;

fn main() {
    // let byte = b'A'; // u8 only
    // let binary = 0b1111_0000;
    // let octa/* l */ = 0o77;
    // let hex = 0xff;
    // let decimal = 98_222;

    match PROGRAM {
        Program::TypeValues => type_values::new(),
        Program::IntegerOverflow => integer_overflow::new(),
        Program::Operators => operators::new(),
        Program::Floating => floating::new(),
        Program::Tuples => tuples::new()
    }
}
