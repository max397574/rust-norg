use std::convert::From;

#[derive(Debug, Eq, PartialEq)]
enum BasicTokenType {
    Character,
    Space,
    Newline,
    Special,
    LinkOpen,
    LinkClose,
}

impl From<char> for BasicTokenType {
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
pub struct BasicToken {
    token_type: BasicTokenType,
    char: char,
}

impl BasicToken {
    fn new(input_char: char) -> Self {
        Self {
            token_type: input_char.into(),
            char: input_char,
        }
    }
}

pub fn tokenize(input: &str) -> impl Iterator<Item = BasicToken> + '_ {
    input.chars().map(BasicToken::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! token {
        ($token_type:ident, $char:expr) => {
            BasicToken {
                token_type: BasicTokenType::$token_type,
                char: $char,
            }
        };
    }

    #[test]
    fn validate_tokenizer() {
        let mut token_iterator = tokenize("*/_,-% \t\n{a}");
        assert_eq!(token_iterator.next(), Some(token!(Special, '*')));
        assert_eq!(token_iterator.next(), Some(token!(Special, '/')));
        assert_eq!(token_iterator.next(), Some(token!(Special, '_')));
        assert_eq!(token_iterator.next(), Some(token!(Special, ',')));
        assert_eq!(token_iterator.next(), Some(token!(Special, '-')));
        assert_eq!(token_iterator.next(), Some(token!(Special, '%')));
        assert_eq!(token_iterator.next(), Some(token!(Space, ' ')));
        assert_eq!(token_iterator.next(), Some(token!(Space, '\t')));
        assert_eq!(token_iterator.next(), Some(token!(Newline, '\n')));
        assert_eq!(token_iterator.next(), Some(token!(LinkOpen, '{')));
        assert_eq!(token_iterator.next(), Some(token!(Character, 'a')));
        assert_eq!(token_iterator.next(), Some(token!(LinkClose, '}')));
        assert_eq!(token_iterator.next(), None);
    }
}
