use std::cmp::PartialEq;
use std::rc::Rc;
use crate::lexer::{Token, TokenType};
use crate::lexer::TokenType::*;
use crate::parser::Node::*;

#[derive(Debug)]
pub enum Node {
    MethodCall(Rc<Token>, Vec<Node>),
    Array(Vec<Node>),
    Member(Rc<Node>, Box<Node>),
    VarDec(Rc<Token>, Box<Node>),
    VarAssign(Box<Node>, Box<Node>),
    Literal(Rc<Token>),
    Identifier(Rc<Token>),
    ExprStatement(Box<Node>),
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Rc<Token>>,
    pub(crate) statements: Vec<Rc<Node>>,
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
    }

    fn statement(&mut self) -> Option<Node> {
        match self.token.token_type() {
            Punctuator => {None}
            Number => {None}
            StringLit => {None}
            Word => {
                Some(ExprStatement(Box::new(self.expression().unwrap())))
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
            _ => {None}
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
                    val => { None } // These cases should be handled when this is called from other places, such as there being a (), and this running into the )
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
            Word => {
                let id = self.token.clone();
                let mut l_member = match self.next().value() {
                    "(" => {
                        self.next(); // Skips the (
                        let mut args = Vec::new();
                        loop {
                            match self.expression() {
                                Some(expr) => {
                                    args.push(expr);
                                }
                                None => {
                                    self.next();
                                    break
                                }
                            }
                            if self.token.value() != "," {
                                self.next();
                                break
                            }
                            self.next();
                        }
                        MethodCall(id, args)
                    }
                    str => {
                        Node::Identifier(id)
                    } // Should be handled below
                };

                while self.token.value() == "." {
                    self.next();
                    match self.expression() {
                        Some(expr) => {
                            l_member = Member(Rc::new(l_member), Box::new(expr));
                        }
                        None => {
                            // println!("Expected expression at: {:?}", self.token)
                        }
                    }
                }
                Some(l_member)
            }
            Operator => {
                println!("Operator?");
                None
            }
            Keyword => {
                println!("Keyword?");
                None
            }
            EndOfFile => {
                println!("Reached end of tokens..");
                None
            }
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