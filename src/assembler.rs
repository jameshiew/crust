use ast::{Program, Expression, FunctionDeclaration, Statement};
use std::fmt::Write;

pub fn assemble(ast: Program) -> String {
    let mut assembly = String::new();
    match ast {
        Program::Main(function_declaration) => match function_declaration {
            FunctionDeclaration::Function(name, statement) => {
                writeln!(&mut assembly, ".globl _{}", name).unwrap();
                writeln!(&mut assembly, "_{}:", name).unwrap();
                match statement {
                    Statement::Return(expression) => {
                        match expression {
                            Expression::Constant(constant) => {
                                writeln!(&mut assembly, "movl\t${}, %eax", constant).unwrap();
                            }
                        }
                        writeln!(&mut assembly, "ret").unwrap();
                    }
                }
            }
        }
    }
    assembly
}
