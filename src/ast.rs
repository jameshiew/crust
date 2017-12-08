use lexer::{Token, Keyword, Operator};
use std::result::Result;
use std::marker::Sized;
use std::collections::VecDeque;


#[derive(Debug)]
pub struct ParseError {
    message: String,
    token: Token,
}

impl ParseError {
    fn new(message: String, token: Token) -> ParseError {
        ParseError { message, token }
    }
}

pub trait Parser {
    fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, ParseError> where Self: Sized;
}

#[derive(Debug)]
pub enum Expression {
    UnOp(Operator, Box<Expression>),
    Constant(i64),
}

impl Parser for Expression {
    fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, ParseError> {
        let token = tokens.pop_front().unwrap();
        match token {
            Token::Operator(operator) => {
                Ok(
                    Expression::UnOp(
                        operator, Box::new(Expression::parse(tokens).unwrap())
                    )
                )
            }
            Token::IntegerLiteral(i) => {
                let int = i.parse::<i64>().unwrap();
                Ok(Expression::Constant(int))
            },
            _ => Err(
                ParseError::new("Expected constant".into(), token)
            ),
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

impl Parser for Statement {
    fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, ParseError> {
        let token = tokens.pop_front().unwrap();
        match token {
            Token::Keyword(keyword) => match keyword {
                Keyword::Return => {
                    let statement = Statement::Return(Expression::parse(tokens).unwrap());
                    let t = tokens.pop_front().unwrap();
                    if t != Token::Semicolon {
                        return Err(
                            ParseError::new("';' expected".into(), t)
                        );
                    }
                    return Ok(statement);
                },
                _ => Err(
                    ParseError::new("Return keyword expected".into(), token)
                ),
            },
            _ => Err(
                ParseError::new("Keyword expected".into(), token)
            ),
        }
    }
}

#[derive(Debug)]
pub enum FunctionDeclaration {
    Function(String, Statement),
}

impl Parser for FunctionDeclaration {
    fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, ParseError> {
        let token = tokens.pop_front().unwrap();
        match token {
            Token::Keyword(keyword) => match keyword {
                Keyword::Int => {
                    let token = tokens.pop_front().unwrap();
                    match token {
                        Token::Identifier(name) => {
                            let t = tokens.pop_front().unwrap();
                            if t != Token::OpenParenthesis {
                                return Err(
                                    ParseError::new("'(' expected".into(), t)
                                );
                            }
                            // function arguments will go here
                            let t = tokens.pop_front().unwrap();
                            if t != Token::CloseParenthesis {
                                return Err(
                                    ParseError::new("')' expected".into(), t)
                                );
                            }
                            let t = tokens.pop_front().unwrap();
                            if t != Token::OpenBrace {
                                return Err(
                                    ParseError::new("'{' expected".into(), t)
                                );
                            }
                            let function_declaration = FunctionDeclaration::Function(
                                name.clone(),
                                Statement::parse(tokens).unwrap(),
                            );
                            let t = tokens.pop_front().unwrap();
                            if t != Token::CloseBrace {
                                return Err(
                                    ParseError::new("'}' expected".into(), t)
                                );
                            }
                            return Ok(function_declaration);
                        },
                        _ => Err(
                            ParseError::new("Function name expected".into(), token)
                        ),
                    }
                },
                _ => Err(
                    ParseError::new("Return type of function declaration expected".into(), token)
                )
            },
            _ => Err(
                ParseError::new("Keyword expected".into(), token)
            ),
        }
    }
}

#[derive(Debug)]
pub enum Program {
    Main(FunctionDeclaration),
}

impl Parser for Program {
    fn parse(tokens: &mut VecDeque<Token>) -> Result<Self, ParseError> {
        // only expecting one 'main' function for now
        Ok(Program::Main(FunctionDeclaration::parse(tokens).unwrap()))
    }
}
