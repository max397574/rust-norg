use std::convert::From;

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
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
    input.chars().map(SimpleToken::new).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! token {
        ($token_type:ident, $char:expr) => {
            SimpleToken {
                token_type: SimpleTokenType::$token_type,
                char: $char,
            }
        };
    }

    #[test]
    fn validate_tokenizer() {
        assert_eq!(
            tokenize("*/_,-% \t\n{a}"),
            vec![
                token!(Special, '*'),
                token!(Special, '/'),
                token!(Special, '_'),
                token!(Special, ','),
                token!(Special, '-'),
                token!(Special, '%'),
                token!(Space, ' '),
                token!(Space, '\t'),
                token!(Newline, '\n'),
                token!(LinkOpen, '{'),
                token!(Character, 'a'),
                token!(LinkClose, '}'),
            ]
        );
    }
}
