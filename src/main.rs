extern crate regex;

use std::{env, fs, io, process};
use std::io::Read;

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
            let bytes_read;
            match reader.read_to_string(&mut contents) {
                Ok(b) => bytes_read = b,
                Err(error) => {
                    eprintln!("{}", error);
                    process::exit(1);
                }
            };
            println!("{}", contents);
            let tokens = lexer::lex(contents.chars());
            println!("{:?}", tokens);
        }
    }
}
