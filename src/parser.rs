pub struct ParsedToken {
    range: [u32; 2],
    data: ParsedTokenData,
}

enum ParsedTokenData {
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
    content: Vec<ParsedToken>,
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

