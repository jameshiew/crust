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

pub fn lex<'a, I>(stream: I) -> Vec<Token<'a>> where I: Iterator{
    let mut tokens = vec![];
    for character in stream {

    }
    tokens
}