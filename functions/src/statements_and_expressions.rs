use std::any;

// get the type of a value
fn type_of<T>(_: &T) -> String {
    let t = any::type_name::<T>();

    t.to_string() // no rust a última expressão é retornada implicitamente não precisando utilizar a palavra chave "return"
                  // mas não pode conter o ponto e vírgula no final, pois a expressão se
                  // transformará em uma instrução (statement)
}

pub fn new() {
    let x = {
        let y = 10;

        (y * 2) as u8 // perceba que está sem o ponto e vírgula (;) caso você o adicione, obterá um erro pois fazendo isso, você transformou uma expression
                      // em um statement, e um statement não tem retorno
    };

    let t = type_of(&x);

    println!("{x} {t}");
}

