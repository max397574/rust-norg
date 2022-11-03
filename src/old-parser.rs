use crate::tokenizer::{self, SimpleToken, SimpleTokenType};
use std::collections::HashMap;

struct Range {
    start: usize,
    end: usize,
}

enum AttachedModifierType {
    Bold,
    Italic,
    Underline,
    Superscript,
    Subscript,
    Strikethrough,
    Comment,
}

struct AttachedModifier {
    char: char,
    mod_type: AttachedModifierType,
    content: Vec<Token>,
}

enum LinkType {
    URL,
}

struct Link {
    link_type: LinkType,
    content: String,
}

enum TokenData {
    Word(String),
    Space(u32),
    SoftBreak,
    ParagraphBreak,
    Link(Link),
    AttachedMOdifier(AttachedModifier),
}

pub struct Token {
    range: Range,
    data: TokenData,
}

struct UnclosedAttachedModifier {
    attached_modifier: AttachedModifier,
    index: usize,
    start: usize,
}

pub fn parse(input: &str, simpleTokens: Vec<tokenizer::SimpleToken>) -> Vec<Token> {
    let input = input.to_string();
    let mut tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let mut start: usize;
    let mut is_paragraph_break = false;
    let mut is_on_new_line = true;
    let mut states: HashMap<char, Vec<UnclosedAttachedModifier>> = HashMap::new();
    while i < simpleTokens.len() {
        let token = &simpleTokens[i];
        start = i;
        match token.token_type {
            SimpleTokenType::Character => {
                while i + 1 < simpleTokens.len()
                    && matches!(simpleTokens[i + 1].token_type, SimpleTokenType::Character)
                {
                    i += 1;
                }
                tokens.push(Token {
                    range: Range { start, end: i + 1 },
                    data: TokenData::Word(input[start..i + 1].to_string()),
                })
            }
            SimpleTokenType::Space => {
                while i + 1 < simpleTokens.len() {
                    match simpleTokens[i + 1].token_type {
                        SimpleTokenType::Space => {
                            i += 1;
                            tokens.push(Token {
                                range: Range { start, end: i + 1 },
                                data: TokenData::Space((i - start + 1).try_into().unwrap()),
                            })
                        }
                        _ => break,
                    }
                }
            }
            SimpleTokenType::Newline => {
                if (i + 1) < simpleTokens.len() {
                    if let SimpleTokenType::Newline = simpleTokens[i + 1].token_type {
                        is_paragraph_break = true;
                        i += 1;
                    }
                }
                tokens.push(Token {
                    range: Range { start, end: i + 1 },
                    data: if is_paragraph_break {
                        TokenData::ParagraphBreak
                    } else {
                        TokenData::SoftBreak
                    },
                });
                is_on_new_line = true;
                continue;
            }
            SimpleTokenType::Special => {
                let can_be_attached_modifier =
                    i + 1 < simpleTokens.len() && simpleTokens[i + 1].char != token.char;
                let can_be_opening_modifier = i > 0
                    && (matches!(&simpleTokens[i - 1].token_type, SimpleTokenType::Space)
                        || matches!(&simpleTokens[i - 1].token_type, SimpleTokenType::Newline));
                let can_be_closing_modifier = i + 1 < simpleTokens.len()
                    && (matches!(simpleTokens[i + 1].token_type, SimpleTokenType::Space)
                        || matches!(simpleTokens[i + 1].token_type, SimpleTokenType::Newline));
                // TODO: add
                let mut unclosedAttachedMods: Vec<UnclosedAttachedModifier> = Vec::new();

                if can_be_attached_modifier && can_be_opening_modifier {
                    unclosedAttachedMods.push(UnclosedAttachedModifier {
                        attached_modifier: AttachedModifier {
                            char: token.char,
                            mod_type: AttachedModifierType::Bold,
                            content: unimplemented!(),
                        },
                        index: tokens.len(),
                        start,
                    })
                } else if can_be_attached_modifier
                    && can_be_closing_modifier
                    && !unclosedAttachedMods.is_empty()
                {
                    // let attached_modifier_opener=unclosedAttachedMods.
                }
            }
            _ => {}
        }
        is_on_new_line = false;
        i += 1;
    }
    tokens
}
