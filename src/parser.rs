use crate::tokenizer::{BasicToken, BasicTokenType};

#[derive(Debug, Eq, PartialEq)]
pub struct ParsedToken {
    range: [u32; 2],
    data: ParsedTokenData,
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
                let mut word = String::new();
                word.push(basic_token.char);

                while let Some(next_token) =
                    basic_tokens.next_if(|x| x.token_type == BasicTokenType::Character)
                {
                    word.push(next_token.char);
                    char_counter += 1;
                }
                parsed_tokens.push(ParsedToken {
                    range: [line_counter, char_counter],
                    data: ParsedTokenData::Word(word),
                })
            },
            BasicTokenType::Space => todo!(),
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
        ($range:expr, $data:expr) => {
            ParsedToken {
                range: $range,
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
                [0, 4],
                ParsedTokenData::Word(String::from("neorg"))
            ))
        );
        assert_eq!(token_iter.next(), None);
    }
}
