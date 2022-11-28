pub fn new() {
    let x = 10;

    // isto funciona porque o if é uma expressão
    let result = if x % 3 == 0 {
        3
    } else if x % 4 == 0 {
        4
    } else if x % 5 == 0 {
        5
    } else {
        -1
    };

    println!("{}", result);
}
