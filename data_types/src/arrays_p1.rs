use std::io;

pub fn new() {
    let numbers = [1, 2, 3, 4, 5];

    println!("Informe um índice: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Falha ao tentar ler o valor");

    let index: usize = index
        .trim()
        .parse()
        .expect("O índice precisa ser um número inteiro");

    let element = numbers[index];

    println!("The value at index {index} is {element}");

    // enquanto o usuário digitar um valor menor do que o tamanho do array
    // o programa irá funcionar com sucesso
    // mas caso ele informe um valor que está fora do range do array
    // o programa irá gerar um panic e parar a execução do programa
    // pois o rust faz essa verificação
    // o que garante que uma memória inválida
    //
    // o que nos previne de um buffer overflow por exemplo
    // o que muitas outras linguagens não fazem esse tipo de tratamento por padrão
}
