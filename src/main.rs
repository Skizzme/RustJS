#[allow(dead_code,unused_variables)]
use std::time::Instant;

mod lexer;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let st = Instant::now();
    let data = std::fs::read(args.get(1).unwrap()).unwrap();

    let mut parser = lexer::Lexer::new(data.as_slice());
    parser.process();
    let et = Instant::now();
    for t in parser.tokens.iter() {
        println!("{:?}", t);
    }
    println!("{:?}, {}", et-st, parser.tokens.len());
}
