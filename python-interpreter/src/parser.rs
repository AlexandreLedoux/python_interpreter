use crate::Token;
use crate::Identifier;
use crate::Value::*;

pub fn is_identifier(chars: &Vec<char>) -> bool {
    if chars.len() == 0 {
        return false;
    } else {
        if chars[0].is_ascii_alphabetic() {
            if chars.len() > 1 {
                for i in 1..(chars.len()) {
                    if !chars[i].is_ascii_alphanumeric() {
                        return false;
                    }
                }

                return true;
            }

            return true;
        }

        return false;
    }
}

pub fn parse(line: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut prev_chars: Vec<char> = Vec::new();
    for c in line.chars() {
        let identifier = is_identifier(&prev_chars);
        if identifier {
            let t = Token::new(Identifier::IDENTIFIER, Str(prev_chars.iter().clone().collect::<String>()));
            tokens.push(t);
            prev_chars = Vec::new();
        } else {
            prev_chars.push(c);
        }
    }

    tokens
}


