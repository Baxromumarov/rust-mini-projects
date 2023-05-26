use std::io;

fn main() {
    // loop {
        let option = read_option();
        let temperature = read_temperature();

        if option == 1 {
            println!("Temperature: {}", (temperature * 1.8) + 32.0);
        } else if option == 2 {
            println!("Temperature: {}", (temperature - 32.0) * (5.0 / 9.0));
        } else {
            println!("You entered wrong option")
        }
    // }
}

fn read_option() -> u32 {
    println!("Choose one option\n1. Celsius to Fahrenheit\n2. Fahrenheit to Celsius\n0. QUIT");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Failed to parse")
        }
    };
    return input;
}

fn read_temperature() -> f32 {
    let mut temperature = String::new();

    println!("Enter Temperature");

    io::stdin().read_line(&mut temperature)
        .expect("Failed to read line. In temperature");

    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Failed to parse")
        }
    };
    return temperature;
}