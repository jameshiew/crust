use lexer;
use std::result::Result;

#[derive(Debug)]
pub enum Expression {
    Constant(i32),
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum FunctionDeclaration {
    Function(String, Statement),
}

#[derive(Debug)]
pub enum Program {
    Main(FunctionDeclaration),
}

pub fn parse(tokens: &Vec<lexer::Token>) -> Result<Program, &str> {
    Ok(
        Program::Main(
            FunctionDeclaration::Function(
                String::from("test"),
                Statement::Return(
                    Expression::Constant(1),
                )
            )
        )
    )
}