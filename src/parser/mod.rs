pub mod llgparser;
pub mod operator;

use std::fmt::Debug;
use crate::grammar::GrammaSymbol;

pub trait Parser<T> {
    fn parse(&self, tokens: &[T]) -> ParseResult<T> where T : PartialEq + Clone;
}

#[derive(Debug)]
pub struct ParseTree<T> {
    value: GrammaSymbol<T>, // This thing probably should not be an gramma sybols
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

#[derive(Debug, Clone)]
pub struct ParseErr {
    trace_log: Vec<String>, // Better add a trace log
}

impl ParseErr {
    pub fn new(message: &str) -> Self {
        ParseErr {
            trace_log: vec![ String::from(message) ]
        }
    }

    pub fn forward(&self, message: &str) -> ParseErr {
        let mut ret = self.clone();
        
        ret.trace_log.push(String::from(message));
        
        return ret;
    }
}

pub type ParseResult<T> = Result<ParseRes<T>, ParseErr>;