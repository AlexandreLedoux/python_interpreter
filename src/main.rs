use python_interpreter::token::token::*;
use python_interpreter::token::types::*;
use python_interpreter::parser::parser::parse;
use std::fs;

fn main() {
    match fs::read_to_string(String::from("input.py")) {
        Ok(content) => {
            let tokens: Vec<Token> = tokenize(content);
        },
        Err(e) => panic!("Impossible d'ouvrir le fichier.")
    }

}
