mod show_most_level_colors;
mod statements_and_expressions;

pub enum Program {
    MostLevelColors,
    StatementsAndExpressions
}

const PROGRAM_RUNNING: Program = Program::StatementsAndExpressions;

fn main() {
    match PROGRAM_RUNNING {
        Program::MostLevelColors => show_most_level_colors::new(),
        Program::StatementsAndExpressions => statements_and_expressions::new(),
    }
}
