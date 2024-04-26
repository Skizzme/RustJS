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
    ListMember(Box<Node>, Box<Node>), // Identifier, Indexer EX list, i+1
    BinaryExpr(Box<Node>, Rc<Token>, Box<Node>), // Left, operator, Right
    UnaryExpr(Box<Node>, Rc<Token>), // Left, operator
    ForLoop(Box<Node>, Box<Node>, Box<Node>), // Var, condition, increment
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
            match self.statement() {
                Some(s) => {
                    self.statements.push(Rc::new(s));
                }
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
                match self.expression() {
                    Some(expr) => {
                        Some(ExprStatement(Box::new(expr)))
                    }
                    None => {None}
                }
            }
            Operator => {None}
            Keyword => {
                match self.token.value() {
                    "var" => {
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
                    }
                    "for" => {
                        self.next(); // Past "for"
                        self.next(); // Past (
                        let var = self.statement().unwrap();
                        if self.token.value() != ";" {
                            return None;
                        }
                        self.next(); // Past ;
                        let condition = self.expression().unwrap();
                        self.next(); // Past ;
                        let inc = self.expression().unwrap();
                        Some(ForLoop(Box::new(var), Box::new(condition), Box::new(inc)))
                    }
                    val => {
                        None
                    }
                }
            }
            _ => {None}
        }
    }

    fn expression(&mut self) -> Option<Node> { // Should always be positioned at the last token of the statement
        let expr = match self.token.token_type() {
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
                match self.next().value() {
                    "(" => {
                        self.next(); // Skips the (
                        let mut args = Vec::new();
                        loop {
                            match self.expression() {
                                Some(expr) => {
                                    args.push(expr);
                                }
                                None => {
                                    break
                                }
                            }
                            if self.token.value() != "," {
                                break
                            }
                            self.next();
                        }
                        Some(FunctionCall(id, args))
                    }
                    "." => {
                        self.next();
                        match self.expression() {
                            Some(expr) => {
                                Some(Member(Rc::new(Identifier(id)), Box::new(expr)))
                            }
                            None => {
                                None
                            }
                        }
                    }
                    str => {
                        Some(Identifier(id))
                    } // Should be handled below
                }
            }
            Operator => {
                println!("Operator? {:?}", self.token);
                None
            },
            Keyword => {
                match self.token.value() {
                    "null" => {
                        self.next();
                        Some(Literal(self.last_token.clone()))
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
        };
        // For binary operators / comparisons
        match expr {
            Some(left) => {
                match self.token.token_type() {
                    Operator => {
                        let op = self.token.clone();
                        match op.value() {
                            "++" => {
                                return Some(UnaryExpr(Box::new(left), op));
                            }
                            _ => {}
                        }
                        self.next();
                        match self.expression() {
                            Some(right) => {
                                Some(BinaryExpr(Box::new(left), op, Box::new(right)))
                            }
                            None => {
                                None
                            }
                        }
                    }
                    _ => {
                        Some(left)
                    }
                }
            }
            None => {
                None
            }
        }
    }

    fn next(&mut self) -> Rc<Token> {
        self.last_token = self.token.clone();
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