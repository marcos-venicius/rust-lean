pub fn new() {
    // criando um array do tipo &str com 12 elementos
    let months: [&str; 12] = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];

    for index in 0..=11 {
        let month = months[index];
        println!("mês {index}: {month}");
    }

    // criando um array que contém 5 elementos com o valor 3
    let numbers = [3; 5];

    for index in 0..numbers.len() {
        let number = numbers[index];

        println!("number {index}: {number}");
    }
}
