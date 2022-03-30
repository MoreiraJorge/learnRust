// C to F -> (celsius  x (9/5)) + 32
// F to C -> (Fahren - 32)/(9/5)
use std::io;
use std::io::prelude::*;

const TEXT: &str = r#"
--------------------------
Choose conversion type:
1 - Convert to Fahrenheit
2 - Convert to Celsius
Press 3 for exit :)
--------------------------
Answer: "#;

fn main() {
    let mut value;

    loop {
        print!("{}", TEXT);
        match read_user_input() {
            0 => {
                println!("");
                println!("Invalid Input!!!");
            }
            1 => {
                print!("Insert value in celsius:");
                value = read_user_input() as f32;
                if value == 0_f32 {
                    println!("");
                    println!("Invalid Input!!!");
                }
                println!("Temperature in Fahrenheit is {} ºF", cel_to_fahr(value))
            }
            2 => {
                print!("Insert value in Fahrenheit:");
                value = read_user_input() as f32;
                if value == 0_f32 {
                    println!("");
                    println!("Invalid Input!!!");
                }
                println!("Temperature in Celsius is {} ºC", fahr_to_cel(value))
            }
            _ => {
                println!("Bye!");
                break;
            }
        };
    }
}

fn read_user_input() -> i32 {
    let mut value = String::new();
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line!");

    value.trim().parse().unwrap_or(0)
}

fn cel_to_fahr(value: f32) -> f32 {
    (value * (9.0 / 5.0)) + 32.0
}

fn fahr_to_cel(value: f32) -> f32 {
    (value - 32.0) / (9.0 / 5.0)
}
