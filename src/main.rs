use std::fmt::{format, Write};
#[allow(dead_code,unused_variables)]
use std::time::Instant;
use crate::lexer::Lexer;
use crate::parser::Parser;

mod lexer;
mod parser;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let data = std::fs::read(args.get(1).unwrap()).unwrap();

    let st = Instant::now();

    let mut lexer = Lexer::new(data.as_slice());
    lexer.process();

    let mut parser = Parser::new(lexer.tokens.clone());
    parser.parse();

    let et = Instant::now();

    let mut output = String::new();
    for statement in parser.statements.iter() {
        output.write_str(format!("{:?}\n", statement).as_str()).unwrap();
        // println!("{:?}", statement);
    }
    std::fs::write("output.txt", output).unwrap();

    println!("{:?}, {}", et-st, lexer.tokens.len());
}
