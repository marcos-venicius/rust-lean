fn is_opening(s: &str) -> bool {
    match s {
        "[" => true,
        "{" => true,
        "(" => true,
         _  => false
    }
}

fn get_close_pair(s: &str) -> &str {
    match s {
        "[" => "]",
        "{" => "}",
        "(" => ")",
         _  => ""
    }
}

pub fn match_brackets(s: &str) -> bool {
     let v: Vec<&str> = s
         .trim()
         .split("")
         .into_iter()
         .filter(|c| c.len() > 0)
         .collect();

     let mut stack: Vec<&str> = Vec::new();

     for e in v {
         let opening = is_opening(e);

         if opening {
             stack.push(e);
         } else {
             if stack.is_empty() {
                 return false;
             }

             let last = stack[stack.len() - 1];

             let pair = get_close_pair(last);

             if e != pair {
                 return false;
             }

             stack.pop();
         }
     }

     return stack.is_empty();
}

#[cfg(test)]
mod tests {
    use crate::match_brackets;

    #[test]
    fn expect_true_1() {
        assert!(match_brackets("()"), "Expected to be valid");
    }

    #[test]
    fn expect_true_2() {
        assert!(match_brackets("[[]]"), "Expected to be valid");
    }

    #[test]
    fn expect_true_3() {
        assert!(match_brackets("(){}[]"), "Expected to be valid");
    }

    #[test]
    fn expect_true_4() {
        assert!(match_brackets("({})[({})]"), "Expected to be valid");
    }

    #[test]
    fn expect_false_1() {
        assert!(!match_brackets("())"), "Expected to be invalid");
    }

    #[test]
    fn expect_false_2() {
        assert!(!match_brackets(")"), "Expected to be invalid");
    }

    #[test]
    fn expect_false_3() {
        assert!(!match_brackets(")("), "Expected to be invalid");
    }

    #[test]
    fn expect_false_4() {
        assert!(!match_brackets("[(])"), "Expected to be invalid");
    }

    #[test]
    fn expect_false_5() {
        assert!(!match_brackets("())({}}{()][]["), "Expected to be invalid");
    }
}

fn main() {
    let tests: [(&str, bool); 9] = [
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

    for test in tests {
        let valid = match_brackets(test.0);

        println!(
            "'{}' DEVE SER {}, RESULTADO: {}",
            test.0, if test.1 { "valido" } else { "invalido" },
            if valid { "valido" } else { "invalido" });
    }
}



