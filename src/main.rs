use std::io;
use std::process;

fn main() {
    println!("Temperature Converter - between Celsius and Fahrenheit.");

    loop {
        println!("Press \n 1- To convert from Celsius to Fahrenheit\n 2- To convert from Fahrenheit to Celsius\n 9- To exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        choice_handler(&choice);
    }
}

fn choice_handler(choice: &str) {
    let choice: u32 = choice.trim().parse().unwrap_or(0);
    match choice {
        1 => temperature_converter("celsius_to_fahrenheit"),
        2 => temperature_converter("fahrenheit_to_celsius"),
        9 => process::exit(1),
        _ => println!("Invalid choice"),
    }
}

fn temperature_converter(key: &str) {
    loop {
        let mut temperature = String::new();
        println!("Enter the temperature:");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line.");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match key {
            "celsius_to_fahrenheit" => println!(
                "{} celsius = {} fahrenheit",
                temperature,
                (temperature * 1.8) + 32.0
            ),
            "fahrenheit_to_celsius" => println!(
                "{} fahrenheit = {} celsius",
                temperature,
                ((temperature - 32.0) / 1.8)
            ),
            _ => println!("Invalid conversion"),
        }
        break;
    }
}
