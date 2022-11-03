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

    #[test]
    fn validate_tokenizer() {
        assert_eq!(
            tokenize("*/_,-% \t\n{a}"),
            vec![
                SimpleToken {
                    token_type: SimpleTokenType::Special,
                    char: '*'
                },
                SimpleToken {
                    token_type: SimpleTokenType::Special,
                    char: '/'
                },
                SimpleToken {
                    token_type: SimpleTokenType::Special,
                    char: '_'
                },
                SimpleToken {
                    token_type: SimpleTokenType::Special,
                    char: ','
                },
                SimpleToken {
                    token_type: SimpleTokenType::Special,
                    char: '-'
                },
                SimpleToken {
                    token_type: SimpleTokenType::Special,
                    char: '%'
                },
                SimpleToken {
                    token_type: SimpleTokenType::Space,
                    char: ' '
                },
                SimpleToken {
                    token_type: SimpleTokenType::Space,
                    char: '\t'
                },
                SimpleToken {
                    token_type: SimpleTokenType::Newline,
                    char: '\n'
                },
                SimpleToken {
                    token_type: SimpleTokenType::LinkOpen,
                    char: '{'
                },
                SimpleToken {
                    token_type: SimpleTokenType::Character,
                    char: 'a'
                },
                SimpleToken {
                    token_type: SimpleTokenType::LinkClose,
                    char: '}'
                }
            ]
        );
    }
}
