pub struct Token {
    range: [u32; 2],
    data: TokenData,
}

enum TokenData {
    Word(String),
    Space(u32),
    SoftBreak,
    ParagraphBreak,
    Link(Link),
    AttachedModifier(AttachedModifier),
}

struct Link {
    variant: LinkType,
    content: String,
}

enum LinkType {
    Url,
}

struct AttachedModifier {
    char: char,
    variant: AttachedModifierType,
    content: Vec<Token>,
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

pub fn parse(tokens: Vec<crate::tokenizer::SimpleToken>) -> Vec<Token> {
    vec![]
}
