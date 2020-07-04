use std::io::{stdin, stdout, Write}; //Like include in C++ or using in C#

fn main() {
    print!("Enter a number: ");
    stdout().flush().unwrap(); //If we don't do this, the message appears after we enter our number.
    let first_num : i32 = {
        let mut input : String = String::new(); //We need to allocate memory
        stdin().read_line(&mut input).unwrap(); //Get number. unwrap is if the program encounter a problem we throw exeption

        input.trim().parse::<i32>().unwrap_or(0) //String to int. We not need to put ; at the end. Basically in OOP we make for example "return a + b" but here it become "a + b"
    };

    //WIP
}
