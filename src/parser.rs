use std::rc::Rc;
use crate::lexer::{Token, TokenType};
use crate::lexer::TokenType::*;
use crate::parser::Node::{Array, Literal, Member, MethodCall, VarDec};

#[derive(Debug)]
pub enum Node {
    MethodCall(Rc<Token>, Vec<Node>),
    Array(Vec<Node>),
    Member(Rc<Token>, Box<Node>),
    VarDec(Rc<Token>, Box<Node>),
    VarAssign(Box<Node>, Box<Node>),
    Literal(Rc<Token>),
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Rc<Token>>,
    statements: Vec<Rc<Node>>,
    index: usize,
    token: Rc<Token>,
    last_token: Rc<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Rc<Token>>) -> Parser {
        let t = tokens.get(0).unwrap().clone();
        Parser {
            tokens,
            statements: Vec::new(),
            index: 0,
            token: t,
            last_token: Rc::new(Token::default()),
        }
    }

    pub fn parse(&mut self) {
        while self.index < self.tokens.len() {
            let st = self.statement();
            match st {
                Some(s) => {
                    self.statements.push(Rc::new(s));}
                None => {}
            }
            self.next();
        }
        for statement in self.statements.iter() {
            println!("{:?}", statement);
        }
    }

    fn statement(&mut self) -> Option<Node> {
        match self.token.token_type() {
            Punctuator => {None}
            Number => {None}
            StringLit => {None}
            Identifier => {
                None
            }
            Operator => {None}
            Keyword => {
                if self.token.value() == "var" {
                    let id = self.next();

                    if self.next().value() == "=" {
                        self.next();
                        match self.expression() {
                            None => {
                                None
                            }
                            Some(expr) => {
                                Some(VarDec(id.clone(), Box::new(expr)))
                            }
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    fn expression(&mut self) -> Option<Node> { // Should always position the index directly after whatever the expression contains
        match self.token.token_type() {
            Punctuator => {
                match self.token.value() {
                    "[" => {
                        self.next();
                        let mut items = Vec::new();
                        loop {
                            items.push(self.expression().unwrap());
                            if self.token.value() != "," {
                                break;
                            }
                            self.next();
                        }
                        self.next();
                        Some(Array(items))
                    }
                    val => {None}
                }
            }
            Number => {
                let t = self.token.clone();
                self.next();
                Some(Literal(t))
            }
            StringLit => {
                let t = self.token.clone();
                self.next();
                Some(Literal(t))
            }
            Identifier => {
                let id = self.token.clone();
                match self.next().value() {
                    "." => {
                        self.next();
                        Some(Member(id, Box::new(self.expression().unwrap())))
                    }
                    "(" => {
                        self.next(); // Skips the (
                        let mut args = Vec::new();
                        loop {
                            match self.expression() {
                                Some(expr) => {
                                    args.push(expr);
                                }
                                None => {}
                            }
                            if self.token.value() != "," {
                                break
                            }
                            self.next();
                        }
                        self.next();
                        Some(MethodCall(id, args))
                        // while self.next()
                    }
                    str => {None}
                }
            }
            Operator => {None}
            Keyword => {None}
        }
    }

    fn next(&mut self) -> Rc<Token> {
        self.index += 1;
        if self.index >= self.tokens.len() {
            self.token = Rc::new(Token::default());
        } else {
            self.token = self.tokens.get(self.index).unwrap().clone();
        }
        self.token.clone()
    }

    fn lookahead(&mut self) -> &Rc<Token> {
        self.tokens.get(self.index+1).unwrap()
    }
}