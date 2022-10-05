use std::io;

fn main() {
    println!("Provide scale name F for Fahrenheit or C for Celsius");
    let mut temperature_scale = String::new();
    io::stdin()
        .read_line(&mut temperature_scale)
        .expect("Failed to read input");

    println!("Provide temperature");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read input");
    let temperature: f32 = temperature
        .parse()
        .expect("Cannot parse string {temperature}");

    let result = if temperature_scale == "C" {
        celsius_to_fahrenheit(temperature);
    } else if temperature_scale == "F" {
        fahrenheit_to_celsius(temperature);
    };
}

fn fahrenheit_to_celsius(temperature: f32) {
    5.0 / 9.0 * (temperature - 32.0);
}

fn celsius_to_fahrenheit(temperature: f32) {
    1.8 * temperature + 32.0;
}
