use crate::token::types::*;

fn get_space(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    let mut i: usize = start_index;
    while i < chars.len() && chars[i] == ' ' { i += 1; }
    let n: usize = i - start_index;
    (TokenType::WhiteSpace(n), n)
}

fn get_identifier(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    let mut i = start_index;
    while i < chars.len() && chars[i].is_ascii_alphanumeric() { i += 1; }
    let n: usize = i - start_index;
    (TokenType::Identifier(chars[start_index..n].iter().clone().collect::<String>()), n)
}

fn get_str(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    let mut i = start_index;
    if chars[i] != '"' { 
        return (TokenType::Str(String::new()), 0);
    }
    i += 1;
    while i < chars.len() && chars[i].is_ascii_alphanumeric() { 
        i += 1;
    }
    if chars[i] != '"' {
        return (TokenType::Str(String::new()), 0);
    }
    let n: usize = i - start_index - 1;
    (TokenType::Str(chars[start_index..n].iter().clone().collect::<String>()), n)
}

fn get_int(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    let mut i = start_index;
    while i < chars.len() && chars[i].is_ascii_digit() { i += 1; }
    let n: usize = i - start_index;
    let val: u32 = (chars[start_index..n].iter().clone().collect::<String>()).parse().unwrap();
    (TokenType::Int(val), n)
}

fn get_plus(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    if chars[start_index] == '+' {
        (TokenType::Plus, 1)    
    } else {
        (TokenType::Plus, 0)
    }
}

fn get_minus(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    if chars[start_index] == '-' { 
        (TokenType::Minus, 1)
    } else { 
        (TokenType::Minus, 0)
    }
}

pub fn tokenize(line: String) -> Vec<Token> {
    let chars: Vec<char> = line.chars().collect();
    let tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
  
    while i < chars.len() {
        let space = get_space(&chars, 0);
        let identifier = get_identifier(&chars, 0);
        let str_ = get_str(&chars, 0);
        let int = get_int(&chars, 0);
        let plus = get_plus(&chars, 0);
        let minus = get_minus(&chars, 0);

        i += 1;
    }

    // dbg!(identifier, str_, int, plus, minus);

    tokens
}

