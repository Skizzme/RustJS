
use std::rc::Rc;
use crate::lexer::TokenType::*;

#[allow(dead_code,unused_variables)]
#[derive(Debug, Clone)]
enum TokenType {
    Punctuator,
    Number,
    StringLit,
    Identifier,
    Operator,
    Keyword,
}

impl Default for TokenType {
    fn default() -> Self {
        StringLit
    }
}

#[allow(dead_code,unused_variables)]
#[derive(Debug, Default, Clone)]
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

impl Token {
    fn new(value: String, token_type: TokenType, start_index: usize, end_index: usize, start_line: usize, end_line: usize, start_column: usize, end_column: usize,) -> Self {
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
            char: '\r',
            last_char: '\r',
            token: Rc::new(Token::default()),
            tokens: Vec::new(),
        }
    }

    pub fn process(&mut self) {

        'char : while self.index < self.data.len() {
            self.new_token(Identifier);

            match self.char {
                '"' => { // String literals inside double quotes
                    self.update_token();
                    while self.next() != '"' && self.last_char != '\\' {
                        self.update_token();
                    }
                    self.update_token();
                    self.end_token(StringLit);
                }
                ',' | '(' | ')' | '{' | '}' | '[' | ']' | '.' | ';' => { // Punctuators
                    self.update_token();
                    self.end_token(Punctuator);
                }
                '=' | '+' | '-' | '<' | '>' | '*' | '/' | '!' => { // Operators
                    self.update_token();
                    if self.next() == '=' {
                        self.update_token();
                    }
                    self.end_token(Operator);
                }
                _char => {
                    while self.char.is_alphabetic() { // Any words/identifiers
                        self.update_token();
                        self.next();
                        if !self.char.is_alphabetic() {
                            let tok = Rc::get_mut(&mut self.token).unwrap();
                            match tok.value.as_str() {
                                "var" | "function" | "for" | "if" | "null" => {
                                    self.end_token(Keyword);
                                }
                                _val => {
                                    self.end_token(Identifier);
                                }
                            }
                            continue 'char;
                        }
                    }
                }
            }
            self.next();
        }
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
        // match m {
        //     None => {}
        //     Some(m) => {
                m.token_type = t;
                m.end_line = self.line;
                m.end_index = self.index;
                m.end_column = self.column;
                self.tokens.push(self.token.clone());
            // }
        // }
    }
}
