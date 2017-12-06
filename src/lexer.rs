#[derive(Debug)]
enum Keyword {
    Int,
    Return,
}

#[derive(Debug)]
pub enum Token<'a> {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    Keyword(Keyword),
    Identifier(&'a str),
    IntegerLiteral(&'a str),
}

pub fn lex<'a>(stream: String) -> Vec<Token<'a>> {
    vec![]
}