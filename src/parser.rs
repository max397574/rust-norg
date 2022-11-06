use crate::tokenizer::{BasicToken, BasicTokenType};
use std::convert::From;

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
    AttachedModifier(AttachedModifierType),
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

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
enum AttachedModifierType {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Verbatim,
    Superscript,
    Subscript,
    Math,
    Variable,
    Comment,
}

impl AttachedModifierType {
    fn new(special_char: char) -> Self {
        match special_char {
            '*' => Self::Bold,
            '/' => Self::Italic,
            '_' => Self::Underline,
            '-' => Self::Strikethrough,
            '|' => Self::Spoiler,
            '`' => Self::Verbatim,
            '^' => Self::Superscript,
            ',' => Self::Subscript,
            '$' => Self::Math,
            '=' => Self::Variable,
            '+' => Self::Comment,
            _ => unreachable!("Tokenizer won't provide unrecognized character"),
        }
    }
}

impl From<AttachedModifierType> for char {
    fn from(attached_modifier_type: AttachedModifierType) -> Self {
        match attached_modifier_type {
            AttachedModifierType::Bold => '*',
            AttachedModifierType::Italic => '/',
            AttachedModifierType::Underline => '_',
            AttachedModifierType::Strikethrough => '-',
            AttachedModifierType::Spoiler => '|',
            AttachedModifierType::Verbatim => '`',
            AttachedModifierType::Superscript => '^',
            AttachedModifierType::Subscript => ',',
            AttachedModifierType::Math => '$',
            AttachedModifierType::Variable => '=',
            AttachedModifierType::Comment => '+',
        }
    }
}

pub fn parse<I>(basic_tokens: I) -> Vec<ParsedToken>
where
    I: Iterator<Item = BasicToken>,
{
    let mut parsed_tokens: Vec<ParsedToken> = vec![];
    let mut basic_tokens = basic_tokens.peekable();
    let mut line_counter: u32 = 0;
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
                while basic_tokens
                    .next_if(|x| x.token_type == BasicTokenType::Space)
                    .is_some()
                {
                    char_counter += 1;
                }
                let end_position = Position::new(line_counter, char_counter);
                parsed_tokens.push(ParsedToken {
                    range: [start_position, end_position],
                    data: ParsedTokenData::Space,
                });
            }
            BasicTokenType::LineBreak => {
                char_counter = 0;
                let start_position = Position::new(line_counter, char_counter);
                if basic_tokens
                    .next_if(|x| x.token_type == BasicTokenType::LineBreak)
                    .is_some()
                {
                    line_counter += 2;
                    let end_position = Position::new(line_counter, char_counter);
                    parsed_tokens.push(ParsedToken {
                        range: [start_position, end_position],
                        data: ParsedTokenData::ParagraphBreak,
                    });
                } else {
                    line_counter += 1;
                    let end_position = Position::new(line_counter, char_counter);
                    parsed_tokens.push(ParsedToken {
                        range: [start_position, end_position],
                        data: ParsedTokenData::SoftBreak,
                    });
                }
            }
            BasicTokenType::Special => {
                let start_position = Position::new(line_counter, char_counter);
                char_counter += 1;
                let end_position = Position::new(line_counter, char_counter);
                let attached_modifier_type = AttachedModifierType::new(basic_token.char);
                parsed_tokens.push(ParsedToken {
                    range: [start_position, end_position],
                    data: ParsedTokenData::AttachedModifier(attached_modifier_type),
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

    #[test]
    fn linebreaks() {
        let mut soft_break_iter = parse(tokenize("\n")).into_iter();
        assert_eq!(
            soft_break_iter.next(),
            Some(parsed_token!([0, 0], [1, 0], ParsedTokenData::SoftBreak))
        );
        assert_eq!(soft_break_iter.next(), None);
        drop(soft_break_iter);

        let mut hard_break_iter = parse(tokenize("\n\n")).into_iter();
        assert_eq!(
            hard_break_iter.next(),
            Some(parsed_token!(
                [0, 0],
                [2, 0],
                ParsedTokenData::ParagraphBreak
            ))
        );
        assert_eq!(hard_break_iter.next(), None);
        drop(hard_break_iter);

        let mut combined_break_iter = parse(tokenize("\n\n\n")).into_iter();
        assert_eq!(
            combined_break_iter.next(),
            Some(parsed_token!(
                [0, 0],
                [2, 0],
                ParsedTokenData::ParagraphBreak
            ))
        );
        assert_eq!(
            combined_break_iter.next(),
            Some(parsed_token!([2, 0], [3, 0], ParsedTokenData::SoftBreak))
        );
    }

    #[test]
    fn attached_modifier_recognition() {
        let mut token_iter = parse(tokenize("*/_-|`^,$=+")).into_iter();
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 0],
                [0, 1],
                ParsedTokenData::AttachedModifier(AttachedModifierType::new('*'))
            ))
        );
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 2],
                [0, 3],
                ParsedTokenData::AttachedModifier(AttachedModifierType::new('/'))
            ))
        );
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 4],
                [0, 5],
                ParsedTokenData::AttachedModifier(AttachedModifierType::new('_'))
            ))
        );
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 6],
                [0, 7],
                ParsedTokenData::AttachedModifier(AttachedModifierType::new('-'))
            ))
        );
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 8],
                [0, 9],
                ParsedTokenData::AttachedModifier(AttachedModifierType::new('|'))
            ))
        );
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 10],
                [0, 11],
                ParsedTokenData::AttachedModifier(AttachedModifierType::new('`'))
            ))
        );
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 12],
                [0, 13],
                ParsedTokenData::AttachedModifier(AttachedModifierType::new('^'))
            ))
        );
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 14],
                [0, 15],
                ParsedTokenData::AttachedModifier(AttachedModifierType::new(','))
            ))
        );
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 16],
                [0, 17],
                ParsedTokenData::AttachedModifier(AttachedModifierType::new('$'))
            ))
        );
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 18],
                [0, 19],
                ParsedTokenData::AttachedModifier(AttachedModifierType::new('='))
            ))
        );
        assert_eq!(
            token_iter.next(),
            Some(parsed_token!(
                [0, 20],
                [0, 21],
                ParsedTokenData::AttachedModifier(AttachedModifierType::new('+'))
            ))
        );
        assert_eq!(token_iter.next(), None);
    }
}
