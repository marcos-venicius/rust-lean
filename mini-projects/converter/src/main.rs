mod converter;

fn main() {
    let fahrenheit = 32.0;
    let celcius = -17.78;

    let f_c = converter::from_fahrenheit_to_celcius(fahrenheit);
    let c_f = converter::from_celcius_to_fahrenheit(celcius);

    println!("C -> F: {celcius} -> {c_f}");
    println!("F -> C: {fahrenheit} -> {f_c}");
}
