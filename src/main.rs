use std::io;
fn main() {
    println!(
        "Enter your numerical input, and it will be displayed as celsius, and converted to fahrenheit and kelvin"
    );
    temperaturecalculator();
}
fn prompt() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let numbers: f64 = input.trim().parse().expect("Not a number.");
    numbers
}
fn temperaturecalculator() {
    let celsius: f64 = prompt();
    let fahrenheit: f64 = celsius * 9.0 / 5.0 + 32.0;
    let kelvin: f64 = celsius + 273.15;
    println!(
        "Celsius: {}, fahrenheit: {}, kelvin: {}",
        celsius, fahrenheit, kelvin
    )
}
