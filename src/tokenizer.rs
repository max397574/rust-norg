use std::convert::From;

#[derive(Debug)]
pub enum SimpleTokenType {
    Character,
    Space,
    Newline,
    Special,
    LinkOpen,
    LinkClose,
}

impl From<char> for SimpleTokenType {
    fn from(input_char: char) -> Self {
        match input_char {
            '\t' | ' ' => Self::Space,
            '\n' => Self::Newline,
            '*' | '/' | '_' | ',' | '-' | '%' => Self::Special,
            '{' => Self::LinkOpen,
            '}' => Self::LinkClose,
            _ => Self::Character,
        }
    }
}

#[derive(Debug)]
pub struct SimpleToken {
    pub token_type: SimpleTokenType,
    pub char: char,
}

impl SimpleToken {
    fn new(input_char: char) -> Self {
        Self {
            token_type: input_char.into(),
            char: input_char,
        }
    }
}

pub fn tokenize(input: &str) -> Vec<SimpleToken> {
    let mut tokens: Vec<SimpleToken> = Vec::new();
    for input_char in input.chars() {
        tokens.push(SimpleToken::new(input_char));
    }
    tokens
}
