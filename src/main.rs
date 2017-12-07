#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::{env, fs, io, process};
use std::io::{Read, Write};

mod assembler;
mod ast;
mod lexer;

fn main() {
    for (i, arg) in env::args().enumerate() {
        if i == 0 {
            continue
        }
        if i == 1 {  // filename
            let source_code_filehandle;
            match fs::File::open(arg.clone()) {
                Ok(file) => source_code_filehandle = file,
                Err(error) => {
                    eprintln!("{}", error);
                    process::exit(1);
                }
            };
            let mut reader = io::BufReader::new(source_code_filehandle);
            let mut contents = String::new();
            match reader.read_to_string(&mut contents) {
                Ok(_) => {
                    println!("Contents: {}", contents);
                    let mut lexer = lexer::Lexer::new();
                    let tokens = lexer.lex(contents.chars());
                    println!("Tokens: {:?}", tokens);
                    let abstract_syntax_tree = ast::parse(tokens).unwrap();
                    println!("AST: {:?}", abstract_syntax_tree);
                    let assembly = assembler::assemble(abstract_syntax_tree);
                    println!("Assembly: {:?}", assembly);
                    {
                        use std::fmt::Write;

                        let mut base_filename = arg.clone();
                        base_filename.truncate(arg.len() - 2);

                        let mut assembly_filename = base_filename.clone();
                        write!(assembly_filename, ".s").unwrap();
                        let mut assembly_filehandle = fs::File::create(assembly_filename.clone()).unwrap();
                        assembly_filehandle.write_all(assembly.as_bytes()).unwrap();

                        use std::process::Command;

                        Command::new("gcc")
                            .arg("-m32")
                            .arg(assembly_filename.clone())
                            .arg("-o")
                            .arg(base_filename.clone())
                            .output()
                            .unwrap();
                    }
                }
                Err(error) => {
                    eprintln!("{}", error);
                    process::exit(1);
                }
            };
        }
    }
}
