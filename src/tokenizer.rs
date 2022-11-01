#[derive(Debug)]
pub enum SimpleTokenType {
    Character,
    Space,
    Newline,
    Special,
    LinkOpen,
    LinkClose,
}

#[derive(Debug)]
pub struct SimpleToken {
    pub token_type: SimpleTokenType,
    pub char: char,
}

pub fn tokenize(input: &str) -> Vec<SimpleToken> {
    let mut tokens: Vec<SimpleToken> = Vec::new();
    for input_char in input.chars() {
        match input_char {
            '\t' | ' ' => {
                tokens.push(SimpleToken {
                    token_type: SimpleTokenType::Space,
                    char: input_char,
                });
            }
            '\n' => {
                tokens.push(SimpleToken {
                    token_type: SimpleTokenType::Newline,
                    char: input_char,
                });
            }
            '*' | '/' | '_' | ',' | '-' | '%' => {
                tokens.push(SimpleToken {
                    token_type: SimpleTokenType::Special,
                    char: input_char,
                });
            }
            '{' => {
                tokens.push(SimpleToken {
                    token_type: SimpleTokenType::LinkOpen,
                    char: input_char,
                });
            }
            '}' => {
                tokens.push(SimpleToken {
                    token_type: SimpleTokenType::LinkClose,
                    char: input_char,
                });
            }
            _ => {
                tokens.push(SimpleToken {
                    token_type: SimpleTokenType::Character,
                    char: input_char,
                });
            }
        }
    }
    tokens
}
