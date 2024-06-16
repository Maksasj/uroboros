pub mod llgparser;
pub mod operator;

use std::fmt::Debug;
use crate::grammar::GrammaSymbols;

pub trait Parser<T> {
    fn parse(&self, tokens: &[T]) -> ParseResult<T> where T : PartialEq + Clone;
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

#[derive(Debug)]
pub struct ParseRes<T> {
    pub tree: Option<Box<ParseTree<T>>>,
    pub consumed: usize
}

#[derive(Debug)]
pub struct ParseErr {
    message: String, // Better add a trace log
}

impl ParseErr {
    pub fn new(message: &str) -> Self {
        ParseErr {
            message: String::from(message)
        }
    }
}

pub type ParseResult<T> = Result<ParseRes<T>, ParseErr>;