use std::any::Any;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::rc::Rc;
use crate::lexer::{Token, TokenType};
use crate::lexer::TokenType::*;
use crate::parser::Node::*;

#[allow(unused)]
#[derive(Debug)]
pub enum Node {
    FunctionCall(Box<Node>, Vec<Node>),
    List(Vec<Node>),
    ListMember(Box<Node>, Box<Node>), // Identifier, Indexer EX list, i+1
    // Object(HashMap<Box<Node>, >) // TODO
    Member(Rc<Node>, Box<Node>),
    VarDec(Rc<Token>, Box<Option<Node>>),
    VarAssign(Box<Node>, Box<Node>), // Identifier, value
    Literal(Rc<Token>),
    Identifier(Rc<Token>),
    ExprStatement(Box<Node>),
    FuncExpression(Rc<Token>, Vec<Rc<Token>>, Box<Node>), // The token of the "function" word, params, block statement
    Block(Vec<Node>),
    BinaryExpr(Box<Node>, Rc<Token>, Box<Node>), // Left, operator, Right
    UnaryExpr(Box<Node>, Rc<Token>), // Left, operator
    ForLoop(Box<Node>, Box<Node>, Box<Node>), // Var, condition, increment
    IfStatement(Box<Node>, Box<Node>), // Condition, block
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Rc<Token>>,
    pub(crate) statements: Vec<Rc<Node>>,
    index: usize,
    token: Rc<Token>,
    last_token: Rc<Token>,
}

#[allow(unused)]
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
                None => {
                    // println!("Skipping over: {:?}", self.token);
                    self.next();
                }
            }
        }
    }

    fn statement(&mut self) -> Option<Node> {
        let mut stat = match self.token.token_type() {
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
                match self.expression() {
                    Some(expr) => {
                        Some(ExprStatement(Box::new(expr)))
                    }
                    None => {
                        println!("what do: {:?}", self.token);
                        None
                    }
                }
            }
            Operator => {None}
            Keyword => {
                match self.token.value() {
                    "var" => {
                        let id = self.next();

                        if self.next().value() == "=" {
                            self.next();
                            println!("none {:?}", self.token);
                            match self.bin_expr() {
                                None => {
                                    None
                                }
                                Some(expr) => {
                                    Some(VarDec(id.clone(), Box::new(Some(expr))))
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
                        let condition = self.bin_expr().unwrap();
                        self.next(); // Past ;
                        match self.bin_expr() {
                            Some(inc) => {Some(ForLoop(Box::new(var), Box::new(condition), Box::new(inc)))}
                            None => {None}
                        }
                    }
                    "if" => {
                        self.next(); // Past if
                        self.next(); // Past (
                        let condition = Box::new(self.bin_expr().unwrap());
                        self.next();
                        Some(IfStatement(condition, Box::new(self.statement().unwrap())))
                    }
                    _val => {
                        None
                    }
                }
            }
            _ => {

                println!("what do: {:?}", self.token);
                None
            }
        };
        if stat.is_some() && self.token.value() == "," {
            self.next();
            let mut stats = Vec::new();
            stats.push(stat.unwrap());
            while self.token.value() != ";" {
                match self.statement() {
                    Some(st) => {
                        stats.push(st)
                    }
                    None => {
                        self.next();
                    }
                }
            }
            stat = Some(Block(stats));
        }
        stat
    }

    fn expression(&mut self) -> Option<Node> { // Should always be positioned at the last token of the statement
        match self.token.token_type() {
            Punctuator => {
                match self.token.value() {
                    "[" => {
                        self.next();
                        let mut items = Vec::new();
                        loop {
                            match self.expression() {
                                Some(ex) => {
                                    items.push(ex);
                                    if self.token.value() != "," {
                                        break;
                                    }
                                    self.next();}

                                None => {
                                    break
                                }
                            }
                        }
                        self.next();
                        Some(List(items))
                    }
                    "(" => {
                        self.next();
                        let expr = self.bin_expr();
                        self.next();
                        expr
                    }
                    _val => { None } // These cases should be handled when this is called from other places, such as there being a (), and this running into the )
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
                let mut ex = Identifier(id);
                self.next();
                loop {
                    match self.token.value() {
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
                            self.next();
                            ex = FunctionCall(Box::new(ex), args);
                            break
                        }
                        "." => {
                            self.next();
                            match self.expression() {
                                Some(expr) => {
                                    ex = Member(Rc::new(ex), Box::new(expr));
                                }
                                None => {
                                    // None
                                    break
                                }
                            }
                        }
                        "[" => {
                            self.next();
                            let member = self.expression();
                            if member.is_some() {
                                ex = ListMember(Box::new(ex), Box::new(member.unwrap()));
                            }
                            self.next();
                        }
                        "=" => {
                            self.next();
                            match self.bin_expr() {
                                Some(bin_expr) => {
                                    ex = VarAssign(Box::new(ex), Box::new(bin_expr))
                                }
                                None => {}
                            }

                        }
                        _str => {
                            break
                        }
                    }
                }
                return Some(ex)
            }
            Operator => {
                let op = self.token.clone();
                self.next();
                match self.expression() {
                    Some(expr) => {
                        Some(UnaryExpr(Box::new(expr), op))
                    }
                    None => {
                        None
                    }
                }
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
                                self.next();
                                match self.token.token_type() {
                                    Word => {
                                        params.push(self.token.clone());
                                        self.next();
                                    }
                                    _ => {}
                                }
                            }
                            self.next(); // Go past the )
                            match self.statement() {
                                Some(stat) => {
                                    Some(FuncExpression(e, params, Box::new(stat)))
                                }
                                None => {
                                    None
                                }
                            }
                        } else {
                            None
                        }
                    }
                    _val => {
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

    fn bin_expr(&mut self) -> Option<Node> {
        match self.expression() {
            Some(left) => {
                let mut e = left;
                let mut op = self.token.clone();
                match op.value() {
                    "++" | "--" => {
                        return Some(UnaryExpr(Box::new(e), op));
                    }
                    _ => {}
                }
                while self.token.is_operator() {
                    let right = match self.token.value() {
                        "||" | "&&" => {
                            self.next();
                            self.bin_expr()
                        }
                        _ => {
                            self.next();
                            self.expression()
                        }
                    };
                    match right {
                        Some(right) => {
                            e = BinaryExpr(Box::new(e), op, Box::new(right));
                        }
                        None => {
                        }
                    }
                    op = self.token.clone();
                }
                Some(e)
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