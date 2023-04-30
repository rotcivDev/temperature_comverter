use std::io;

fn main() {
    println!("Temperature Converter - between Celsius and Fahrenheit.");
    loop {
        println!("Press \n 1- To convert from C to F\n 2- To convert from F to C\n9- To exit");

        const EXIT_CHOICE: u32 = 9;
        const INVALID_CHOICE: u32 = 0;

        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line.");

        let choice = choice_handler(&unit);

        if choice == INVALID_CHOICE {
            println!("invalid input");
        }
        if choice == EXIT_CHOICE {
            break;
        }
        if choice == 1 {
            celsius_to_fahrenheit();
        }
        if choice == 2 {
            fahrenheit_to_celsius();
        }
    }
}
fn choice_handler(value: &String) -> u32 {
    let value: u32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if value == 1 {
        return value;
    }

    if value == 2 {
        return value;
    }

    if value == 9 {
        println!("exiting");
        return value;
    }
    0
}
fn celsius_to_fahrenheit() {
    loop {
        let mut temperature = String::new();
        println!("Enter the temperature in Celsius");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line.");

        let in_celsius: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let in_fahrenheit: f32 = (in_celsius * 1.8) + 32.0;
        println!("{} celsius = {} fahrenheit", in_celsius, in_fahrenheit);
        break;
    }
}
fn fahrenheit_to_celsius() {
    loop {
        let mut temperature = String::new();
        println!("Enter the temperature in Fahrenheit");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line.");

        let in_fahrenheit: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let in_celsius: f32 = ((in_fahrenheit - 32.0) / 1.8).try_into().unwrap();

        println!("{} fahrenheit = {} celsius", in_fahrenheit, in_celsius);
        break;
    }
}
