use std::usize;

use crate::{grammar::{GrammaSymbols, Production}, sigma};

use super::{ParseErr, ParseRes, ParseResult, ParseTree, Parser};

pub struct LLGParser<T> {
    grammar: Vec<Production<T>>,
    entry: GrammaSymbols<T>
}

impl<T> LLGParser<T> {
    pub fn new(grammar: Vec<Production<T>>, entry: GrammaSymbols<T>) -> Self {
        LLGParser {
            grammar: grammar,
            entry: entry,
        }
    }

    fn parse_internal(&self, tokens: &[T], head: &mut usize, grammar: &Vec<Production<T>>, entry: &GrammaSymbols<T>) -> ParseResult<T> where T : PartialEq + Clone {
        match entry {
            GrammaSymbols::<T>::NonTerminal(_) => {
                for prod in grammar.iter() {
                    if prod.left != entry.clone() { 
                        continue; 
                    }

                    let backup: usize = *head;
                    let mut tree: Option<ParseTree<T>> = None;
                    let mut consumed: usize = 0;

                    for right in prod.right.iter() {
                        consumed = 0;
                        let mut childs: Vec<Box<ParseTree<T>>> = vec![];

                        let mut suc: bool = true;

                        for symbol in right.iter() {
                            let prod = self.parse_internal(tokens, head, grammar, &symbol.clone());

                            if prod.is_err() {
                                *head = backup;
                                suc = false;
                                break;
                            }

                            let parse_res = prod.unwrap();
                            match parse_res.tree {
                                Some(child) => {
                                    consumed += parse_res.consumed;
                                    childs.push(child)
                                }
                                None => { },
                            }
                        }

                        if suc == false {
                            continue;
                        }
                        
                        tree = Some(ParseTree {
                            value: entry.clone(),
                            childs: match childs.is_empty() {
                                true => None,
                                false => Some(childs)
                            }
                        });

                        break;
                    }

                    return match tree {
                        Some(t) => {
                            Ok(ParseRes {
                                tree: match t.childs {
                                    Some(_) => Some(Box::new(t)),
                                    None => None
                                },
                                consumed: consumed
                            })
                        },
                        None => Err(ParseErr::new("Tree error")),
                    }
                }

                return Err(ParseErr::new("Failed to do something"));
            },
            GrammaSymbols::<T>::Terminal(token) => {
                if *head >= tokens.len() {
                    return Err(ParseErr::new("Expected terminal but input is empty"));
                }

                if token.clone() == tokens[*head] {
                    *head += 1;

                    let tree: ParseTree<T> = ParseTree {
                        value: entry.clone(),
                        childs: None
                    };

                    return Ok( ParseRes {
                        tree: Some(Box::new(tree)),
                        consumed: 1
                    });
                }
        
                return Err(ParseErr::new("Expected terminal but got something else")); 
            },
            sigma!() => {
                return Ok(ParseRes {
                    tree: None,
                    consumed: 0
                });
            }
        }
    }
}

impl<T> Parser<T> for LLGParser<T> {
    fn parse(&self, tokens: &[T]) -> ParseResult<T> where T : PartialEq + Clone {
        // Rust ? Whyyyyyy ?
        let grammar: Vec<Production<T>> = self.grammar.clone();
        let entry: GrammaSymbols<T> = self.entry.clone();
        let mut head = 0;

        return self.parse_internal(tokens, &mut head, &grammar, &entry);
    }
}