pub mod llgparser;

use std::fmt::Debug;
use crate::grammar::GrammaSymbols;

pub trait Parser<T> {
    fn parse(&mut self, tokens: &Vec<(T, String)>) -> ParseResult<T> where T : PartialEq + Clone;
}

#[derive(Debug)]
pub struct ParseTree<T> {
    value: GrammaSymbols<T>, // This thing probably should not be an gramma sybols
    childs: Option<Vec<Box<ParseTree<T>>>>,
}

impl<T> ParseTree<T> where T : Debug {
    fn debug_log_depth(&self, depth: u32) {
        for _ in 0..depth {
            print!("  ");
        }

        println!("{:?}", self.value);

        match &self.childs {
            Some(childs) => {
                for child in childs.iter() {
                    for _ in 0..depth {
                        print!("  ");
                    }
                    
                    child.debug_log_depth(depth + 1);
                }
            },
            None => { },
        }
    }

    pub fn debug_log(&self) {
        self.debug_log_depth(0);
    }
}

pub type ParseResult<T> = Result<Option<Box<ParseTree<T>>>, ()>;

pub struct TokenParser<T> {
    expect: T
}

impl<T : 'static> TokenParser<T> {
    pub fn new(expect: T) -> Box<dyn Parser<T>> {
        return Box::new(TokenParser {
            expect: expect
        });
    }
}

impl<T> Parser<T> for TokenParser<T> {
    fn parse(&mut self, tokens: &Vec<(T, String)>) -> ParseResult<T> where T : PartialEq + Clone {
        if tokens.is_empty() {
            return Err(());
        }

        if tokens[0].0 == self.expect {
            return Ok(Some(Box::new(ParseTree {
                value: GrammaSymbols::Terminal(self.expect.clone()),
                childs: None
            })));
        }

        Err(())
    }
}

pub struct OrParser<T> {
    left: Box<dyn Parser<T>>,
    right: Box<dyn Parser<T>>
}

impl<T : 'static> OrParser<T> {
    pub fn new(left: Box<dyn Parser<T>>, right: Box<dyn Parser<T>>) -> Box<dyn Parser<T>> {
        let or: OrParser<T> = OrParser {
            left: left,
            right: right
        };

        return Box::new(or);
    }
}

impl<T> Parser<T> for OrParser<T> {
    fn parse(&mut self, tokens: &Vec<(T, String)>) -> ParseResult<T> where T : PartialEq + Clone {
        let res1: Result<Option<Box<ParseTree<T>>>, ()> = (*self.left).parse(tokens);

        if res1.is_ok()  {
            return res1;
        }

        let res2: Result<Option<Box<ParseTree<T>>>, ()> = (*self.right).parse(tokens);

        if res2.is_ok()  {
            return res2;
        }

        Err(())
    }
}
