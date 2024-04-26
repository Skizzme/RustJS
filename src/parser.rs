use std::cmp::PartialEq;
use std::rc::Rc;
use crate::lexer::{Token, TokenType};
use crate::lexer::TokenType::*;
use crate::parser::Node::*;

#[derive(Debug)]
pub enum Node {
    FunctionCall(Rc<Token>, Vec<Node>),
    Array(Vec<Node>),
    Member(Rc<Node>, Box<Node>),
    VarDec(Rc<Token>, Box<Node>),
    VarAssign(Box<Node>, Box<Node>),
    Literal(Rc<Token>),
    Identifier(Rc<Token>),
    ExprStatement(Box<Node>),
    FuncExpression(Rc<Token>, Vec<Rc<Token>>, Box<Node>), // The token of the "function" word, params, block statement
    Block(Vec<Node>),
    ListMember(Box<Node>, Box<Node>) // Identifier, Indexer EX list, i+1
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
            Punctuator => {
                match self.token.value() {
                    "{" => {
                        self.next();
                        let mut statements = Vec::new();
                        while self.token.value() != "}" {
                            match self.statement() {
                                Some(expr) => {
                                    statements.push(expr);
                                }
                                None => {
                                    self.next();
                                }
                            }
                        }
                        self.next();
                        Some(Block(statements))
                    }
                    _ => {
                        None
                    }
                }
            }
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
                let mut member = match self.next().value() {
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
                        FunctionCall(id.clone(), args)
                    }
                    // "." => {
                    //     self.next();
                    //     match self.expression() {
                    //         Some(expr) => {
                    //             Some(Member(Rc::new(Identifier(id)), Box::new(expr)))
                    //         }
                    //         None => {
                    //             None
                    //         }
                    //     }
                    // }
                    str => {
                        Identifier(id.clone())
                    } // Should be handled below
                };

                while self.token.value() == "." {
                    self.next();
                    match self.expression() {
                        Some(expr) => {
                            member = Member(Rc::new(member), Box::new(expr));
                        }
                        None => {
                            // println!("Expected expression at: {:?}", self.token)
                        }
                    }
                }
                Some(member)
            }
            Operator => {
                println!("Operator? {:?}", self.token);
                None
            }
            Keyword => {
                match self.token.value() {
                    "null" => {
                        Some(Literal(self.token.clone()))
                    }
                    // Function expression, like a lambda or var a = function () {}
                    "function" => {
                        let e = self.token.clone();
                        if self.next().value() == "(" {
                            let mut params = Vec::new();
                            while self.token.value() == "(" || self.token.value() == "," {
                                match self.token.token_type() {
                                    Word => {
                                        params.push(self.token.clone())
                                    }
                                    _ => {}
                                }
                                self.next();
                            }
                            self.next(); // Go past the )
                            match self.statement() {
                                Some(stat) => {
                                    Some(FuncExpression(e, params, Box::new(stat)))
                                }
                                None => {
                                    println!("T{:?}", self.token);
                                    None
                                }
                            }
                        } else {
                            None
                        }
                    }
                    val => {
                        None
                    }
                }
            }
            EndOfFile => {
                println!("Reached end of tokens.. {:?}", self.token);
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