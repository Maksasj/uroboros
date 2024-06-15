use std::{marker::PhantomData, process::Child};

use crate::grammar::GrammaSymbols;

use super::{ParseErr, ParseRes, ParseResult, ParseTree, Parser};

#[derive(Debug)]
pub struct Exact<T> {
    expect: T
}

impl<T : 'static> Exact<T> {
    pub fn new(expect: T) -> Box<dyn Parser<T>> {
        return Box::new(Exact {
            expect: expect
        });
    }
}

impl<T> Parser<T> for Exact<T> {
    fn parse(&self, tokens: &Vec<T>) -> ParseResult<T> where T : PartialEq + Clone {
        if tokens.is_empty() {
            return Err(ParseErr::new("Expected token, but input is empty"));
        }
        
        if tokens[0] == self.expect {
            return Ok(ParseRes {
                tree: Some(Box::new(ParseTree {
                    value: GrammaSymbols::Terminal(self.expect.clone()),
                    childs: None
                })),
                consumed: 0
            });
        }

        Err(ParseErr::new("Expected another token"))
    }
}

pub struct Or<T> {
    left: Box<dyn Parser<T>>,
    right: Box<dyn Parser<T>>
}

impl<T : 'static> Or<T> {
    pub fn new(left: Box<dyn Parser<T>>, right: Box<dyn Parser<T>>) -> Box<dyn Parser<T>> {
        return Box::new(Or { left, right });
    }
}

impl<T> Parser<T> for Or<T> {
    fn parse(&self, tokens: &Vec<T>) -> ParseResult<T> where T : PartialEq + Clone {
        let res1= (*self.left).parse(tokens);

        if res1.is_ok()  {
            return res1;
        }

        let res2 = (*self.right).parse(tokens);

        if res2.is_ok()  {
            return res2;
        }

        Err(ParseErr::new("Expected token OR token"))
    }
}

pub struct Many<T> {
    child: Box<dyn Parser<T>>,
}

impl<T : 'static> Many<T> {
    pub fn new(child: Box<dyn Parser<T>>) -> Box<dyn Parser<T>> {
        return Box::new(Many {
            child: child
        });
    }
}

impl<T> Parser<T> for Many<T> {
    fn parse(&self, tokens: &Vec<T>) -> ParseResult<T> where T : PartialEq + Clone {
        let mut childs: Vec<Box<ParseTree<T>>> = vec![];

        loop {
            let child = self.child.parse(tokens);

            match child {
                Ok(c) => childs.push(c.tree.unwrap()),
                Err(_) => break
            }
        }

        return match childs.len() {
            0 => { Err(ParseErr::new("Expected many tokens, but got zero")) }
            _ => {
                Ok(ParseRes {
                    tree: Some(Box::new(ParseTree {
                        value: GrammaSymbols::Sigma, // Todo remove this, Probably this should be a non terminal Many
                        childs: Some(childs)
                    })),
                    consumed: 0
                })
            }
        };
    }
}

pub struct SeqOf<T> {
    childs: Vec<Box<dyn Parser<T>>>,
}

impl<T : 'static> SeqOf<T> {
    pub fn new(childs: Vec<Box<dyn Parser<T>>>) -> Box<dyn Parser<T>> {
        return Box::new(SeqOf { childs });
    }
}

impl<T> Parser<T> for SeqOf<T> {
    fn parse(&self, tokens: &Vec<T>) -> ParseResult<T> where T : PartialEq + Clone {
        let mut childs: Vec<Box<ParseTree<T>>> = vec![];

        for child in self.childs.iter() {
            let res = child.parse(tokens);

            match res {
                Ok(c) => childs.push(c.tree.unwrap()),
                Err(_) => return Err(ParseErr::new("One of child parsers failed"))
            }
        }

        Ok(ParseRes {
            tree: Some(Box::new(ParseTree {
                value: GrammaSymbols::Sigma, // Todo remove this, Probably this should be a non terminal SeqOf
                childs: Some(childs)
            })),
            consumed: 0
        })
    }
}

pub struct Eof<T>(PhantomData<T>);

impl<T : 'static> Eof<T> {
    pub fn new() -> Box<dyn Parser<T>> {
        return Box::new(Eof::<T> { 0: PhantomData });
    }
}

impl<T> Parser<T> for Eof<T> {
    fn parse(&self, tokens: &Vec<T>) -> ParseResult<T> where T : PartialEq + Clone {
        return match tokens.is_empty() {
            true => Ok(ParseRes {
                tree: Some(Box::new(ParseTree {
                    value: GrammaSymbols::Sigma, // Todo remove this, Probably this should be a non terminal Eof
                    childs: None
                })),
                consumed: 0
            }),
            false => Err(ParseErr::new("Expected end of input")),
        };
    }
}


