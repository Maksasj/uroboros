pub mod ll_grammar_parser;

use std::fmt::Debug;
use crate::grammar::GrammaSymbols;

pub trait Parser<T> {
    fn parse(&mut self, tokens: &Vec<(T, String)>) -> ParseResult<T> where T : PartialEq + Clone;
}

#[derive(Debug)]
pub struct ParseTree<T> {
    value: GrammaSymbols<T>,
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
