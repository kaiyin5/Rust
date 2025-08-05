use std::io;

fn main() {
    lyrics_printer();
}

// 1. Convert temperatures between Fahrenheit and Celsius.
// using user prompt
fn fahrenheit_celsius_conversion_cli() -> f64 {
    println!("Please input your temperature.");
    let temperature: f64 = loop {
        let mut temperature_input = String::new();
        io::stdin()
            .read_line(&mut temperature_input)
            .expect("Failed to read line");
        let temperature: f64 = match temperature_input.trim().parse() {
            Ok(float) => float,
            Err(_) => {
                println!("Please input a valid temperature.");
                continue;
            }
        };
        break temperature;
    };
    println!("Please input the temperature scale you want to convert to. (C/F)");
    println!("Type 'C' for Fahrenheit to Celsius conversion.");
    let is_c: bool = loop {
        let mut scale_input = String::new();
        io::stdin()
            .read_line(&mut scale_input)
            .expect("Failed to read line");
        let is_c: bool = match &scale_input.trim().to_uppercase()[..1] {
            "C" => true,
            "F" => false,
            _ => {
                println!("Please input a valid scale. C or F.");
                continue;
            }
        };
        break is_c;
    };
    if is_c {
        5.0 / 9.0 * (temperature - 32.0)
    } else {
        temperature * 1.8 + 32.0
    }
}

// using parameters
fn fahrenheit_celsius_conversion(temperature: f64, temp_type: &str) -> f64 {
    if &temp_type.trim().to_uppercase()[..1] == "C" {
        temperature * 1.8 + 32.0
    } else if &temp_type.trim().to_uppercase()[..1] == "F" {
        5.0 / 9.0 * (temperature - 32.0)
    } else {
        println!("Invalid temperature scale input.\n Unchanged temperature returned.");
        temperature
    }
}

// 2. Generate the nth Fibonacci number.
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

/* 3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
taking advantage of the repetition in the song. */
fn lyrics_printer() {
    let nth: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let christmas_carol_lines: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for i in 0..nth.len() {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            nth[i]
        );
        for j in 0..i + 1 {
            println!("{}", christmas_carol_lines[i - j])
        }
        println!();
    }
}
