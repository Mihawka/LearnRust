mod banquev1;
mod banquev2;
mod banquev3;

use std::io::{stdin, stdout, Write};

fn main() {
    let input = ask_number();
    
    match input {
        1 => banquev1::started(),
        2 => banquev2::started(),
        3 => banquev3::started(),
        _ => main(),
    }    
}

fn ask_number() -> i32 {
    print!("Choisir la version: ");
    stdout().flush().unwrap();
    let input = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input
    };
    match input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => ask_number(),
    } 
}