mod match_brackets;

fn main() {
    let brackets_with_correct_results: [(&str, bool); 9] = [
        ("()", true),
        ("[[]]", true),
        ("(){}[]", true),
        ("({})[({})]", true),
        ("())", false),
        (")", false),
        (")(", false),
        ("[(])", false),
        ("())({}}{()][][", false),
    ];

    for bracket_with_correct_result in brackets_with_correct_results {
        let valid = match_brackets::validate(bracket_with_correct_result.0);

        println!(
            "'{}' DEVE SER {}, RESULTADO: {}",
            bracket_with_correct_result.0,
            if bracket_with_correct_result.1 { "valido" } else { "invalido" },
            if valid { "valido" } else { "invalido" });
    }
}



