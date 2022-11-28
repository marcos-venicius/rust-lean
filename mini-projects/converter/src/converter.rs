pub fn from_fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
    // formula: C = 5 / 9 * (F - 32)

    (5.0 / 9.0 * (fahrenheit - 32.0)).round()
}

pub fn from_celcius_to_fahrenheit(celcius: f32) -> f32 {
    // formula: F = C * (9 / 5) + 32

    (celcius * (9.0 / 5.0) + 32.0).round()
}
