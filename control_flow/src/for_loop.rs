pub fn new() {
    // let mut numbers = [1, 2, 3, 4, 5];

    // forma 1

    // let mut index = 0;

    // multiplicar todos os valores do numbers
    // mais lento pois o rust adiciona um código para verificar se o indice é valído
    // caso não seja ele gera um panic
    // while index < 5 {
    //     numbers[index] = numbers[index] * 2;

    //     index += 1;
    // }
    
    // forma 2
    
    // for number in &mut numbers {
    //     *number = *number * 2;
    // }
    
    // forma 3
    
    // for index in 0..numbers.len() {
    //     numbers[index] = numbers[index] * 2;
    // }

    // mostra todos os valores do numbers
    // for number in numbers {
    //     println!("{number}");
    // }

    // countdown com for

    // rev -> reverse : inverte a lista
    for number in (1..=5).rev() {
        if number == 1 {
            println!("{number} ...");
        } else {
            print!("{number} ");
        }
    }
}
