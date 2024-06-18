use core::fmt;
use std::marker::PhantomData;

use crate::{grammar::GrammaSymbol, sym};

use super::{ParseErr, ParseRes, ParseResult, ParseTree, Parser};

#[derive(Debug)]
pub struct Exact<T> {
    expect: T
}

impl<T : fmt::Debug + 'static> Exact<T> {
    pub fn new(expect: T) -> Box<dyn Parser<T>> {
        return Box::new(Exact { expect });
    }
}

impl<T : fmt::Debug> Parser<T> for Exact<T> {
    fn parse(&self, tokens: &[T]) -> ParseResult<T> where T : PartialEq + Clone {
        if tokens.is_empty() {
            return Err(ParseErr::new(format!("Expected token '{:?}' but input is empty", self.expect).as_str() ));
        }
        
        if tokens[0] == self.expect {
            return Ok(ParseRes {
                tree: Some(Box::new(ParseTree {
                    value: GrammaSymbol::Terminal(self.expect.clone()),
                    childs: None
                })),
                consumed: 1
            });
        }

        Err(ParseErr::new(format!("Expected '{:?}' token but got '{:?}' token", self.expect, tokens[0]).as_str()))
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
    fn parse(&self, tokens: &[T]) -> ParseResult<T> where T : PartialEq + Clone {
        let res1= (*self.left).parse(tokens);

        if res1.is_ok()  {
            return res1;
        }

        let res2 = (*self.right).parse(tokens);

        if res2.is_ok()  {
            return res2;
        }

        Err(ParseErr::new("Left and right or parser branches failed"))
    }
}

pub struct Many<T> {
    child: Box<dyn Parser<T>>,
}

impl<T : 'static> Many<T> {
    pub fn new(child: Box<dyn Parser<T>>) -> Box<dyn Parser<T>> {
        return Box::new(Many { child });
    }
}

impl<T> Parser<T> for Many<T> {
    fn parse(&self, tokens: &[T]) -> ParseResult<T> where T : PartialEq + Clone {
        let mut childs: Vec<Box<ParseTree<T>>> = vec![];
        let mut consumed: usize = 0;

        loop {
            let child = self.child.parse(&tokens[consumed..]);

            match child {
                Ok(c) => {
                    consumed += c.consumed;
                    childs.push(c.tree.unwrap())
                },
                Err(_) => break
            }
        }

        return match childs.len() {
            0 => { Err(ParseErr::new("Expected many matches but matched zero times")) }
            _ => {
                Ok(ParseRes {
                    tree: Some(Box::new(ParseTree {
                        value: sym!("many"),
                        childs: Some(childs)
                    })),
                    consumed: consumed
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
    fn parse(&self, tokens: &[T]) -> ParseResult<T> where T : PartialEq + Clone {
        let mut childs: Vec<Box<ParseTree<T>>> = vec![];
        let mut consumed: usize = 0;

        for child in self.childs.iter() {
            let res = child.parse(&tokens[consumed..]);

            match res {
                Ok(c) => { 
                    consumed += c.consumed;
                    childs.push(c.tree.unwrap())
                }
                Err(error) => return Err(error.forward("Child parser failed"))
            }
        }

        Ok(ParseRes {
            tree: Some(Box::new(ParseTree {
                value: sym!("seq_of"), // Todo remove this, Probably this should be a non terminal SeqOf
                childs: Some(childs)
            })),
            consumed: consumed
        })
    }
}
pub struct Times<T> {
    times: usize,
    child: Box<dyn Parser<T>>,
}

impl<T : 'static> Times<T> {
    pub fn new(times : usize, child: Box<dyn Parser<T>>) -> Box<dyn Parser<T>> {
        return Box::new(Times { times, child });
    }
}

impl<T> Parser<T> for Times<T> {
    fn parse(&self, tokens: &[T]) -> ParseResult<T> where T : PartialEq + Clone {
        let mut childs: Vec<Box<ParseTree<T>>> = vec![];
        let mut consumed: usize = 0;
    
        for _ in 0..self.times {
            let res = self.child.parse(&tokens[consumed..]);

            match res {
                Ok(c) => { 
                    consumed += c.consumed;
                    childs.push(c.tree.unwrap())
                },
                Err(error) => return Err(error.forward("Child parser failed"))
            }
        }

        Ok(ParseRes {
            tree: Some(Box::new(ParseTree {
                value: sym!("times"),
                childs: Some(childs)
            })),
            consumed: consumed
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
    fn parse(&self, tokens: &[T]) -> ParseResult<T> where T : PartialEq + Clone {
        return match tokens.is_empty() {
            true => Ok(ParseRes {
                tree: Some(Box::new(ParseTree {
                    value: sym!("eof"),
                    childs: None
                })),
                consumed: 0
            }),
            false => Err(ParseErr::new("Expected end of input")),
        };
    }
}


