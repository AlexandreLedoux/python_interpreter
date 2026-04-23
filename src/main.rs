use python_interpreter::reader::*;
use python_interpreter::token::token::*;

fn main() {
    let lines = get_string(String::from("input.py"));

    for line in lines {
        match line {
            Ok(val) => tokenize(val),
            Err(_) => panic!("Erreur de la lecture de la ligne")
        };
    }
}
