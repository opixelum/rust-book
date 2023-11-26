use std::io;

const CELSIUS_STR: &str = "celsius";
const FAHRENHEIT_STR: &str = "fahrenheit";
const CELSIUS_SYM: &str = "°C";
const FAHRENHEIT_SYM: &str = "°F";

enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn read_input<T: std::str::FromStr>() -> T {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line from stdin");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again.")
        }
    }
}

fn main() {
    let temperature_unit: TemperatureUnit;

    loop {
        println!("From which temperature unit you want to convert:\n\t1. Celsius\n\t2. Fahrenheit\nEnter your choice:");

        let choice: u8 = read_input();

        temperature_unit = match choice {
            1 => TemperatureUnit::Celsius,
            2 => TemperatureUnit::Fahrenheit,
            _ => {
                println!("Invalid choice. Please enter 1 for Celsius or 2 for Fahrenheit.");
                continue;
            }
        };

        break;
    }

    println!(
        "Enter the {} temperature value to convert in {}:",
        match temperature_unit {
            TemperatureUnit::Celsius => CELSIUS_STR,
            TemperatureUnit::Fahrenheit => FAHRENHEIT_STR
        },
        match temperature_unit {
            TemperatureUnit::Celsius => FAHRENHEIT_STR,
            TemperatureUnit::Fahrenheit => CELSIUS_STR
        }
    );

    let temperature_value: f64 = read_input();

    let converted_temperature = match temperature_unit {
        TemperatureUnit::Celsius => celsius_to_fahrenheit(temperature_value),
        TemperatureUnit::Fahrenheit => fahrenheit_to_celsius(temperature_value)
    };

    println!(
        "{converted_temperature} {}",
        match temperature_unit {
            TemperatureUnit::Celsius => FAHRENHEIT_SYM,
            TemperatureUnit::Fahrenheit => CELSIUS_SYM
        }
    );
}
