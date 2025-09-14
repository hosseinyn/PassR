use passwords::PasswordGenerator;
use std::io::{self, Write};

fn main() {

    println!("PassR | Password generator wrote in rust ");

    // Length input handling

    print!("Enter the length : ");
    io::stdout().flush().unwrap();
    let mut length_input = String::from("") ;

    io::stdin().read_line(&mut length_input).expect("problem in length input");

    let final_length: usize = length_input.trim().parse().expect("not a valid integer");

    let mut is_number : bool = false;
    let mut is_lowercase : bool = false;
    let mut is_uppercase : bool = false;
    let mut is_symbols : bool = false;

    // Numbers input handling

    print!("Have numbers? (y,n) : ");
    io::stdout().flush().unwrap();
    let mut number_input = String::from("") ;

    io::stdin().read_line(&mut number_input).expect("problem in number input");

    if number_input.trim() == "y" {
        is_number = true;
    }

    // Lowercase letters input hanling

    print!("Have lowercase letters? (y,n) : ");
    io::stdout().flush().unwrap();
    let mut lowercase_input = String::from("") ;

    io::stdin().read_line(&mut lowercase_input).expect("problem in lower case input");

    if lowercase_input.trim() == "y" {
        is_lowercase = true;
    }

    // Uppercase letters input hanling

    print!("Have uppercase letters? (y,n) : ");
    io::stdout().flush().unwrap();
    let mut uppercase_input = String::from("") ;

    io::stdin().read_line(&mut  uppercase_input).expect("problem in upper case input");

    if uppercase_input.trim() == "y" {
        is_uppercase = true;
    }

    // Symbols input hanling

    print!("Have symbols? (y,n) : ");
    io::stdout().flush().unwrap();
    let mut symbols_input = String::from("") ;

    io::stdin().read_line(&mut  symbols_input).expect("problem in symbols input");

    if symbols_input.trim() == "y" {
        is_symbols = true;
    }


    let pg = PasswordGenerator {
        length: final_length,
        numbers: is_number,
        lowercase_letters: is_lowercase,
        uppercase_letters: is_uppercase,
        symbols: is_symbols,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };

    println!("\n Generated password : \n {}", pg.generate_one().unwrap());

}