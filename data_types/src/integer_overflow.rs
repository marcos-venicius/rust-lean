pub fn new() {
    let mut u8_overflow: u8 = 255;

    u8_overflow += 1;
    u8_overflow += 1;

    // no modo de debug o programa irá compilar, mas ao executar nós teremos um panic e o programa será encerrado

    // [debug] cargo run

    // por outro lado, no modo release a variável u8_overflow terá seu valor setado para 1 e o programa irá seguir sem problemas
    // [release] cargo run --release

    let mut u8_overflow2: u8 = 255;

    u8_overflow2 += 1_u8.wrapping_add(2); // 2, 0 + 2
    u8_overflow2 += 1_u8.wrapping_add(2); // 5, 2 + 1 + 2

    println!("u8_overflow agora é: {u8_overflow}");
    println!("u8_overflow2 agora é: {u8_overflow2}"); // 5
}
