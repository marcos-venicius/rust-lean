mod integer_overflow;
mod type_values;
mod floating;
mod tuples;
mod arrays;
mod arrays_p1;

pub enum Program {
    IntegerOverflow,
    TypeValues,
    Floating,
    Tuples,
    Arrays,
    ArraysProgram1,
}


const PROGRAM: Program = Program::ArraysProgram1;

fn main() {
    // let byte = b'A'; // u8 only
    // let binary = 0b1111_0000;
    // let octa/* l */ = 0o77;
    // let hex = 0xff;
    // let decimal = 98_222;

    match PROGRAM {
        Program::TypeValues => type_values::new(),
        Program::IntegerOverflow => integer_overflow::new(),
        Program::Floating => floating::new(),
        Program::Tuples => tuples::new(),
        Program::Arrays => arrays::new(),
        Program::ArraysProgram1 => arrays_p1::new()
    }
}
