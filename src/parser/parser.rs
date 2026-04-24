use crate::token::types::TokenType;
use crate::token::types::Token;
use dict::Dict;

fn set(dict: Dict::<String>, identifier: String, token_type: TokenType) {
    // dict.add(identifier, token_type);
}

pub fn parse(tokens: Vec<Vec<Token>>) {
    let mut indent: usize = 0;

    let mut dict = Dict::<String>::new();


}
