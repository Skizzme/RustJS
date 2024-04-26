use std::fmt::{Write};
use std::os::windows::fs::MetadataExt;
use std::time::Duration;
#[allow(dead_code,unused_variables)]
use std::time::Instant;
use crate::lexer::Lexer;
use crate::parser::Parser;

mod lexer;
mod parser;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let path = args.get(1).unwrap();
    let mut last_time = 0u64;
    loop {
        let t = std::fs::metadata(path).expect("TODO: panic message").last_write_time();
        if last_time != t {
            last_time = t;
            let data = std::fs::read(path).unwrap();

            let st = Instant::now();

            let mut lexer = Lexer::new(data.as_slice());
            lexer.process();

            let mut parser = Parser::new(lexer.tokens.clone());
            parser.parse();

            let et = Instant::now();

            let mut output = String::new();
            for statement in parser.statements.iter() {
                output.write_str(format!("{:#?}\n", statement).as_str()).unwrap();
                // println!("{:?}", statement);
            }
            std::fs::write("output.txt", output).unwrap();

            println!("{:?}, {}", et-st, lexer.tokens.len());
        }
        std::thread::sleep(Duration::from_millis(50))
    }
}
