use crate::tokenizer::{BasicToken, BasicTokenType};

#[derive(Debug, Eq, PartialEq)]
pub struct ParsedToken {
    range: [Position; 2],
    data: ParsedTokenData,
}

#[derive(Debug, Eq, PartialEq)]
struct Position {
    line_counter: u32,
    char_counter: u32,
}

impl Position {
    fn new(line_counter: u32, char_counter: u32) -> Self {
        Self {
            line_counter,
            char_counter,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
enum ParsedTokenData {
    Word(String),
    Space,
    SoftBreak,
    ParagraphBreak,
    Link(Link),
    AttachedModifier(AttachedModifier),
}

#[derive(Debug, Eq, PartialEq)]
struct Link {
    variant: LinkType,
    content: String,
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
enum LinkType {
    Url,
}

#[derive(Debug, Eq, PartialEq)]
struct AttachedModifier {
    char: char,
    variant: AttachedModifierType,
    content: Vec<ParsedToken>,
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
enum AttachedModifierType {
    Bold,
    Italic,
    Underline,
    Superscript,
    Subscript,
    Strikethrough,
    Comment,
}

pub fn parse<I>(basic_tokens: I) -> Vec<ParsedToken>
where
    I: Iterator<Item = BasicToken>,
{
    let mut parsed_tokens: Vec<ParsedToken> = vec![];
    let mut basic_tokens = basic_tokens.peekable();
    let line_counter: u32 = 0;
    let mut char_counter: u32 = 0;

    while let Some(basic_token) = basic_tokens.next() {
        match basic_token.token_type {
            BasicTokenType::Character => {
                let start_position = Position::new(line_counter, char_counter);
                let mut word = String::new();
                word.push(basic_token.char);

                while let Some(next_token) =
                    basic_tokens.next_if(|x| x.token_type == BasicTokenType::Character)
                {
                    word.push(next_token.char);
                    char_counter += 1;
                }
                let end_position = Position::new(line_counter, char_counter);
                parsed_tokens.push(ParsedToken {
                    range: [start_position, end_position],
                    data: ParsedTokenData::Word(word),
                })
            }
            BasicTokenType::Space => {
                let start_position = Position::new(line_counter, char_counter);
                while let Some(_) = basic_tokens.next_if(|x| x.token_type == BasicTokenType::Space)
                {
                    char_counter += 1;
                }
                let end_position = Position::new(line_counter, char_counter);
                parsed_tokens.push(ParsedToken {
                    range: [start_position, end_position],
                    data: ParsedTokenData::Space,
                });
            }
            _ => unimplemented!(),
        }
        char_counter += 1;
    }
    parsed_tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::tokenize;

    macro_rules! parsed_token {
        ($start:expr, $end:expr, $data:expr) => {
            ParsedToken {
                range: [
                    Position::new($start[0], $start[1]),
                    Position::new($end[0], $end[1]),
                ],
                data: $data,
            }
        };
    }

    #[test]
    fn word() {
        let mut token_iter = parse(tokenize("neorg")).into_iter();
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 0],
                [0, 4],
                ParsedTokenData::Word(String::from("neorg"))
            ))
        );
        assert_eq!(token_iter.next(), None);
    }

    #[test]
    fn words_with_whitespaces() {
        let mut token_iter = parse(tokenize("neorg parser    ")).into_iter();
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 0],
                [0, 4],
                ParsedTokenData::Word(String::from("neorg"))
            ))
        );
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!([0, 5], [0, 5], ParsedTokenData::Space))
        );
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 6],
                [0, 11],
                ParsedTokenData::Word(String::from("parser"))
            ))
        );
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!([0, 12], [0, 15], ParsedTokenData::Space))
        );
        assert_eq!(token_iter.next(), None);
    }
}
