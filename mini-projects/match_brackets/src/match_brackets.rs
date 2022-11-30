fn is_opening_bracket(s: &str) -> bool {
    match s {
        "[" => true,
        "{" => true,
        "(" => true,
         _  => false
    }
}

fn get_opened_bracket_pair(s: &str) -> &str {
    match s {
        "[" => "]",
        "{" => "}",
        "(" => ")",
         _  => ""
    }
}

pub fn validate(s: &str) -> bool {
     let brackets_list: Vec<&str> = s
         .trim()
         .split("")
         .into_iter()
         .filter(|c| c.len() > 0)
         .collect();

     let mut brackets_stack: Vec<&str> = Vec::new();

     for bracket in brackets_list {
         let opening_bracket = is_opening_bracket(bracket);

         if opening_bracket {
             brackets_stack.push(bracket);
         } else {
             if brackets_stack.is_empty() {
                 return false;
             }

             let last_bracket_on_stack = brackets_stack[brackets_stack.len() - 1];

             let last_bracket_on_stack_pair = get_opened_bracket_pair(last_bracket_on_stack);

             if bracket != last_bracket_on_stack_pair {
                 return false;
             }

             brackets_stack.pop();
         }
     }

     return brackets_stack.is_empty();
}

#[cfg(test)]
mod tests {
    use crate::match_brackets::validate;

    #[test]
    fn expect_true_1() {
        assert!(validate("()"), "Expected to be valid");
    }

    #[test]
    fn expect_true_2() {
        assert!(validate("[[]]"), "Expected to be valid");
    }

    #[test]
    fn expect_true_3() {
        assert!(validate("(){}[]"), "Expected to be valid");
    }

    #[test]
    fn expect_true_4() {
        assert!(validate("({})[({})]"), "Expected to be valid");
    }

    #[test]
    fn expect_false_1() {
        assert!(!validate("())"), "Expected to be invalid");
    }

    #[test]
    fn expect_false_2() {
        assert!(!validate(")"), "Expected to be invalid");
    }

    #[test]
    fn expect_false_3() {
        assert!(!validate(")("), "Expected to be invalid");
    }

    #[test]
    fn expect_false_4() {
        assert!(!validate("[(])"), "Expected to be invalid");
    }

    #[test]
    fn expect_false_5() {
        assert!(!validate("())({}}{()][]["), "Expected to be invalid");
    }
}
