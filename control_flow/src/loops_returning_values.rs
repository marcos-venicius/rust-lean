// código apenas para testes
// poderia ser melhorado
fn factorial(of: &mut i32) -> i32 {
    let mut sum: i32 = 1;
    let mut index: i32 = 1;

    if *of == 0 {
        return 0;
    }

    if *of < 0 {
        *of = *of * -1;
    }

    return loop {
        sum *= index;

        if index == *of {
            break sum; // retorna o valor de sum
        }

        index += 1;
    }
}

pub fn new() {
    // retornando valores de um loop

    let result = factorial(&mut 5); // 120

    println!("5! is {}", result);
}
