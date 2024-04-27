use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::lexer::TokenType::*;

#[allow(dead_code,unused_variables)]
#[derive(Debug, Clone)]
pub enum TokenType {
    Punctuator,
    Number,
    StringLit,
    Word,
    Operator,
    Keyword,
    EndOfFile,
}

impl Default for TokenType {
    fn default() -> Self {
        EndOfFile
    }
}

#[allow(dead_code,unused_variables)]
#[derive(Default, Clone, Debug)]
pub struct Token {
    value: String,
    token_type: TokenType,
    start_index: usize,
    end_index: usize,
    start_line: usize,
    end_line: usize,
    start_column: usize,
    end_column: usize
}

// impl Debug for Token {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_str(format!("Token(\n    value: \"{}\",\n    token_type: {:?}\n)", self.value, self.token_type).as_str())
//     }
// }

#[allow(unused)]
impl Token {
    pub fn new(value: String, token_type: TokenType, start_index: usize, end_index: usize, start_line: usize, end_line: usize, start_column: usize, end_column: usize,) -> Self {
        Token {
            value,
            token_type,
            start_index,
            end_index,
            start_line,
            end_line,
            start_column,
            end_column,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
    pub fn start_index(&self) -> usize {
        self.start_index
    }
    pub fn end_index(&self) -> usize {
        self.end_index
    }
    pub fn start_line(&self) -> usize {
        self.start_line
    }
    pub fn end_line(&self) -> usize {
        self.end_line
    }
    pub fn start_column(&self) -> usize {
        self.start_column
    }
    pub fn end_column(&self) -> usize {
        self.end_column
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    data: &'a [u8],
    index: usize,
    line: usize,
    column: usize,
    char: char,
    last_char: char,
    token: Rc<Token>,
    pub tokens: Vec<Rc<Token>>,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Lexer {
            data,
            index: 0usize,
            line: 0usize,
            column: 0usize,
            char: data[0] as char,
            last_char: '\r',
            token: Rc::new(Token::default()),
            tokens: Vec::new(),
        }
    }

    pub fn process(&mut self) {

        'char : while self.index < self.data.len() {
            self.new_token(Word);

            match self.char {
                // String literals inside double quotes
                '"' => {
                    self.update_token();

                    while self.next() != '"' && self.last_char != '\\' && self.char != '\r' {
                        self.update_token();
                    }

                    self.update_token();
                    self.end_token(StringLit);
                }
                // Punctuators
                '(' | ')' | '{' | '}' | '[' | ']' | ',' | '.' | ';' => {
                    self.update_token();
                    self.end_token(Punctuator);
                }
                // Operators
                '=' | '+' | '-' | '<' | '>' | '*' | '/' | '!' | '&' | '|' => {
                    let char = self.char;
                    self.update_token();
                    self.next();
                    // All negative numbers
                    if char == '-' {
                        if self.char.is_numeric() {
                            self.number();
                            println!("t:{:?}", self.token);
                            continue 'char
                        }
                    }

                    if ((self.char == '+' || self.char == '-' || self.char == '<' || self.char == '>' || self.char == '&' || self.char == '|') && self.char == char) ||
                        (self.char == '=' && (char == '!' || char == '='))
                    {
                        self.update_token();
                        self.next();
                    }
                    self.end_token(Operator);
                    continue 'char;
                }
                char => {
                    // Any words/identifiers
                    if char.is_alphabetic() {
                        while self.char.is_alphabetic() {
                            self.update_token();
                            self.next();
                        }
                        let tok = Rc::get_mut(&mut self.token).unwrap();
                        match tok.value.as_str() {
                            "var" | "function" | "for" | "if" | "null" => {
                                self.end_token(Keyword);
                            }
                            _val => {
                                self.end_token(Word);
                            }
                        }
                        continue 'char;
                    }
                    // Any numbers. EX. 1, 1.2, 1_200
                    if self.char.is_numeric() {
                        self.number();
                        continue 'char;
                    }
                }
            }
            self.next();
        }
    }

    fn number(&mut self) {
        while self.char.is_numeric() || self.char == '.' || self.char == '_' {
            self.update_token();
            self.next();
        }
        self.end_token(Number);
    }

    fn update_token(&mut self) {
        match Rc::get_mut(&mut self.token) {
            None => {}
            Some(m) => {
                m.value.push(self.char);
            }
        }
    }

    fn next(&mut self) -> char {
        self.last_char = self.char;
        self.index += 1;
        self.column += 1;

        if self.index >= self.data.len() {
            self.char = '\r';
            return self.char
        } else {
            self.char = self.data[self.index] as char;
        }

        if self.char == '\n' {
            self.column = 0;
            self.line += 1;
            self.next();
        } else if self.char == '\r' || self.char == '\t' {
            self.next();
        }

        self.char
    }

    fn new_token(&mut self, token_type: TokenType) {
        let t = Rc::new(Token::new(
            "".to_string(),
            token_type,
            self.index,
            self.index,
            self.line,
            self.line,
            self.column,
            self.column,
        ));
        self.token = t;
    }

    fn end_token(&mut self, t: TokenType) {
        let m = Rc::get_mut(&mut self.token).unwrap();

        m.token_type = t;
        m.end_line = self.line;
        m.end_index = self.index;
        m.end_column = self.column;

        self.tokens.push(self.token.clone());
    }
}
