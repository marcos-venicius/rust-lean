pub fn new() {
    // podemos definir um tipo para cada item da tupla
    // eles não necessariamente precisam ser do mesmo tipo
    let color: (u8, u8, u8, f32) = (0, 255, 0, 0.5);

    // desestruturando uma tupla
    let (r, g, b, a) = color;

    println!("RGBA({r}, {g}, {b}, {a})");

    // tambem podemos acessar um valor da tupla usando um periodo
    // `.` seguido do index do valor que queremos acessar

    let alpha = color.3;

    println!("ALPHA: {alpha}");

    // unit 
    let _tuple: () = ();

    let tuple_without_type = (10, 10, "test");

    let (x, y, name) = tuple_without_type;


    println!("TUPLE WITHOUT TYPE: ({x},{y}) \"{name}\"");
}

