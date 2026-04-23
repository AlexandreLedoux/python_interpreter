use crate::token::types::*;

fn get_white_space(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    let mut n: usize = 0;
    while start_index + n < chars.len() && chars[start_index + n] == ' ' { n += 1; }
    (TokenType::WhiteSpace(n), n)
}

fn get_identifier(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    let mut n: usize = 0;
    while start_index + n < chars.len() && chars[start_index + n].is_ascii_alphabetic() { n += 1; }
    (TokenType::Identifier(chars[start_index..(start_index + n)].iter().clone().collect::<String>()), n)
}

fn get_str(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    let mut n: usize = 0;
    if chars[start_index] != '"' { return (TokenType::Str(String::new()), 0); }
    n += 1;
    while start_index + n < chars.len() && chars[start_index + n].is_ascii_alphanumeric() { n += 1; }
    if chars[start_index + n] != '"' { return (TokenType::Str(String::new()), 0); }
    (TokenType::Str(chars[start_index..(start_index + n)].iter().clone().collect::<String>()), n)
}

fn get_int(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    let mut n: usize = 0;
    while start_index + n < chars.len() && chars[start_index + n].is_ascii_digit() { n += 1; }
    match (chars[start_index..(start_index + n)].iter().clone().collect::<String>()).parse() {
        Ok(val) => (TokenType::Int(val), n),
        _ => (TokenType::Int(0), 0)
    }
}

fn get_plus(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    match chars[start_index] {
        '+' => (TokenType::Plus, 1),
        _ => (TokenType::Plus, 0)
    }
}

fn get_minus(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    match chars[start_index] { 
        '-' => (TokenType::Minus, 1),
        _ => (TokenType::Minus, 0)
    }
}

fn get_equal(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    match chars[start_index] {
        '=' => (TokenType::Equal, 1),
        _ => (TokenType::Equal, 0)
    }
}

pub fn tokenize(line: String) -> Vec<Token> {
    let chars: Vec<char> = line.chars().collect();
    let tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
  
    while i < chars.len() {
        println!("{}", i);
        let mut scores: Vec<(TokenType, usize)> = Vec::new();

        scores.push(get_white_space(&chars, i));
        scores.push(get_identifier(&chars, i));
        scores.push(get_str(&chars, i));
        scores.push(get_int(&chars, i));
        scores.push(get_plus(&chars, i));
        scores.push(get_minus(&chars, i));
        scores.push(get_equal(&chars, i));

        // let mut max: (TokenType, usize) = scores[0];
        scores.sort_by(|a, b| a.1.cmp(&b.1));

        dbg!(scores);
        println!("---------");

        i += 1;
    }

    tokens
}

