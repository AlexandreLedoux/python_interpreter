mod reader;
use crate::reader::*;
mod token;
use crate::token::*;
mod parser;
use crate::parser::*;

fn main() {
    let lines = get_string(String::from("input.py"));

    for line in lines {
        match line {
            Ok(val) => {
                println!("{}", val);
                let tokens: Vec<Token> = parse(val);
                for token in tokens {
                    println!("{}", token);
                }
            },
            _ => println!("ERROR")
        }
    }
}
