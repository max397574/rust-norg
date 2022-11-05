use crate::tokenizer::{tokenize, BasicToken, BasicTokenType};

#[derive(Debug, Eq, PartialEq)]
pub struct ParsedToken {
    range: [u32; 2],
    data: ParsedTokenData,
}

#[derive(Debug, Eq, PartialEq)]
enum ParsedTokenData {
    Word(String),
    Space(u32),
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
    I: Iterator<Item = BasicToken>
{
    let mut parsed_tokens: Vec<ParsedToken> = vec![];
    let mut basic_tokens = basic_tokens.peekable();

    while let Some(basic_token) = basic_tokens.next() {
        match basic_token.token_type {
            BasicTokenType::Character => {
                let mut word = String::new();
                word.push(basic_token.char);
                loop {
                    // TODO: break using 1.65 syntax
                    match basic_tokens.peek() {
                        Some(next_token) => {
                            if next_token.token_type == BasicTokenType::Character {
                                word.push(basic_tokens.next().unwrap().char);
                            } else {
                                break
                            }
                        },
                        None => break,
                    }
                }
                parsed_tokens.push(ParsedToken {
                    range: [0, (word.len() - 1) as u32],
                    data: ParsedTokenData::Word(word),
                })
            }
            _ => unimplemented!(),
        }
    }
    parsed_tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word() {
        assert_eq!(parse(tokenize("hi")), vec![
                   ParsedToken {
                       range: [0,1],
                       data: ParsedTokenData::Word("hi".into()),
                   }
        ]);
    }
}
