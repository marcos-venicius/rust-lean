mod show_most_level_colors;

pub enum Program {
    MostLevelColors
}

const PROGRAM_RUNNING: Program = Program::MostLevelColors;

fn main() {
    match PROGRAM_RUNNING {
        Program::MostLevelColors => show_most_level_colors::new(),
    }
}
