use python_interpreter::reader::*;
use python_interpreter::token::token::*;

fn main() {
    let lines = get_string(String::from("input.py"));

    for (i, line) in lines.enumerate() {
        match line {
            Ok(val) => tokenize(val, i),
            Err(_) => panic!("Erreur de la lecture de la ligne")
        };
    }
}
