#[macro_use] extern crate lazy_static;
extern crate regex;

use std::{env, fs, io, process};
use std::io::Read;

mod ast;
mod lexer;

fn main() {
    for (i, arg) in env::args().enumerate() {
        if i == 0 {
            continue
        }
        if i == 1 {  // filename
            let file;
            match fs::File::open(arg) {
                Ok(f) => file = f,
                Err(error) => {
                    eprintln!("{}", error);
                    process::exit(1);
                },
            };
            let mut reader = io::BufReader::new(file);
            let mut contents = String::new();
            match reader.read_to_string(&mut contents) {
                Ok(_) => {
                    println!("Contents: {}", contents);
                    let mut lexer = lexer::Lexer::new();
                    let tokens = lexer.lex(contents.chars());
                    println!("Tokens: {:?}", tokens);
                    println!("Program: {:?}", ast::parse(tokens));
                },
                Err(error) => {
                    eprintln!("{}", error);
                    process::exit(1);
                }
            };
        }
    }
}
