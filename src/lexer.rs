use std::str;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Keyword {
    Int,
    Return,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Negation,
    BitwiseComplement,
    LogicalNegation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    Keyword(Keyword),
    Identifier(String),
    IntegerLiteral(String),
    Operator(Operator),
    Unknown(String),
}

pub struct Lexer {
    tokens: VecDeque<Token>,
    current: Vec<char>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            tokens: VecDeque::new(),
            current: vec!(),
        }
    }

    fn consume(&mut self) {
        if self.current.is_empty() {
            return
        }
        lazy_static! {
            static ref INT_KEYWORD: Regex = Regex::new(r"^int$").unwrap();
            static ref RETURN_KEYWORD: Regex = Regex::new(r"^return$").unwrap();
            static ref IDENTIFIER: Regex = Regex::new(r"^[a-zA-Z]\w*$").unwrap();
            static ref INTEGER_LITERAL: Regex = Regex::new(r"^[0-9]+$").unwrap();
        }
        let string = self.current.iter().clone().collect::<String>();
        let str: &str = string.as_ref();
        let token = match str {
            _ if INTEGER_LITERAL.is_match(str) => Token::IntegerLiteral(string.clone()),
            _ if INT_KEYWORD.is_match(str) => Token::Keyword(Keyword::Int),
            _ if RETURN_KEYWORD.is_match(str) => Token::Keyword(Keyword::Return),
            _ if IDENTIFIER.is_match(str) => Token::Identifier(string.clone()),
            _ => Token::Unknown(string.clone()),
        };
        self.tokens.push_back(token);
        self.current = vec!();
    }

    pub fn lex(&mut self, stream: str::Chars) -> &mut VecDeque<Token> {
        for character in stream {
            match character {
                _ if character.is_whitespace() => {
                    self.consume();
                },
                '{' => {
                    self.consume();
                    self.tokens.push_back(Token::OpenBrace);
                }
                '}' => {
                    self.consume();
                    self.tokens.push_back(Token::CloseBrace);
                }
                '(' => {
                    self.consume();
                    self.tokens.push_back(Token::OpenParenthesis);
                }
                ')' => {
                    self.consume();
                    self.tokens.push_back(Token::CloseParenthesis);
                }
                ';' => {
                    self.consume();
                    self.tokens.push_back(Token::Semicolon);
                }
                '-' => {
                    self.consume();
                    self.tokens.push_back(Token::Operator(Operator::Negation));
                }
                '~' => {
                    self.consume();
                    self.tokens.push_back(Token::Operator(Operator::BitwiseComplement));
                }
                '!' => {
                    self.consume();
                    self.tokens.push_back(Token::Operator(Operator::LogicalNegation));
                }
                _ => self.current.push(character),
            }
        }
        &mut self.tokens
    }
}