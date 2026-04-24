use crate::token::types::*;

fn get_white_space(chars: &Vec<char>, start_index: usize) -> usize {
    let mut n: usize = 0;
    while start_index + n < chars.len() && chars[start_index + n] == ' ' { n += 1; }
    n
}

fn get_newline(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    match chars[start_index] {
        '\n' => (TokenType::Newline, 1),
        _ => (TokenType::Newline, 0)
    }
}

fn get_indent(chars: &Vec<char>, start_index: usize) -> (TokenType, usize) {
    let mut n: usize = 0;
    while start_index + n < chars.len() && chars[start_index + n] == ' ' { n += 1; }
    if n != 0 && n % 4 == 0 {
        (TokenType::Indent, n)
    } else {
        (TokenType::Indent, 0)
    }
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
    while start_index + n < chars.len() && chars[start_index + n] != '"' { n += 1; }
    n += 1;
    if start_index + n - 1 == chars.len() { return (TokenType::Str(String::new()), 0); }
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

pub fn tokenize(content: String) -> Vec<Token> {
    let chars: Vec<char> = content.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let mut line_i: usize = 0;

    while i < chars.len() {
        let mut scores: Vec<(TokenType, usize)> = Vec::new();

        scores.push(get_newline(&chars, i));
        scores.push(get_indent(&chars, i));
        scores.push(get_identifier(&chars, i));
        scores.push(get_str(&chars, i));
        scores.push(get_int(&chars, i));
        scores.push(get_plus(&chars, i));
        scores.push(get_minus(&chars, i));
        scores.push(get_equal(&chars, i));

        scores.sort_by(|a, b| b.1.cmp(&a.1));
        let best_count = scores[0].1;

        if best_count != 0 {
            if scores[0].0 == TokenType::Newline { line_i += 1; }
            let token: Token = Token::new(scores[0].0.clone(), line_i, i);
            tokens.push(token);
            i += best_count;
        } else {
            panic!("Erreur de syntaxe {}:{}", line_i, i);
        }
    }

    dbg!(content);
    dbg!(&tokens);

    tokens
}
