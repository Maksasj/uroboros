use crate::{grammar::{GrammaSymbols, Production}, sigma};

use super::{ParseResult, ParseTree, Parser};

pub struct LLGrammarParser<T> {
    grammar: Vec<Production<T>>,
    entry: GrammaSymbols<T>,
    head: usize
}

impl<T> LLGrammarParser<T> {
    pub fn new(grammar: Vec<Production<T>>, entry: GrammaSymbols<T>) -> Self {
        LLGrammarParser {
            grammar: grammar,
            entry: entry,
            head: 0
        }
    }

    fn parse_internal(&mut self, tokens: &Vec<(T, String)>, grammar: &Vec<Production<T>>, entry: &GrammaSymbols<T>) -> ParseResult<T> where T : PartialEq + Clone {
        match entry {
            GrammaSymbols::<T>::NonTerminal(_) => {
                for prod in grammar.iter() {
                    if prod.left != entry.clone() { 
                        continue; 
                    }

                    let backup: usize = self.head;
                    let mut tree: Option<ParseTree<T>> = None;

                    for right in prod.right.iter() {
                        let mut childs: Vec<Box<ParseTree<T>>> = vec![];
                        let mut suc: bool = true;

                        for symbol in right.iter() {
                            let prod = self.parse_internal(tokens, grammar, &symbol.clone());

                            if prod.is_err() {
                                self.head = backup;
                                suc = false;
                                break;
                            }
                            
                            match prod.unwrap() {
                                Some(child) => childs.push(child),
                                None => { },
                            }
                        }

                        if suc == false {
                            continue;
                        }
                        
                        if childs.is_empty() {
                            tree = Some(ParseTree {
                                value: entry.clone(),
                                childs: None
                            });
                        } else {
                            tree = Some(ParseTree {
                                value: entry.clone(),
                                childs: Some(childs)
                            });
                        }
                    }

                    return match tree {
                        Some(t) => {
                            return match t.childs {
                                Some(_) => Ok(Some(Box::new(t))),
                                None => Ok(None),
                            }
                        },
                        None => Err(()),
                    }
                }

                return Err(())
            },
            GrammaSymbols::<T>::Terminal(token) => {
                if self.head >= tokens.len() {
                    return Err(())
                }

                if token.clone() == tokens[self.head].0 {
                    self.head += 1;

                    let tree: ParseTree<T> = ParseTree {
                        value: entry.clone(),
                        childs: None
                    };

                    return Ok(Some(Box::new(tree)));
                }
        
                return Err(()); 
            },
            sigma!() => {
                return Ok(None);
            }
        }
    }
}

impl<T> Parser<T> for LLGrammarParser<T> {
    fn parse(&mut self, tokens: &Vec<(T, String)>) -> ParseResult<T> where T : PartialEq + Clone {
        // Rust ? Whyyyyyy ?
        let grammar: Vec<Production<T>> = self.grammar.clone();
        let entry: GrammaSymbols<T> = self.entry.clone();

        return self.parse_internal(tokens, &grammar, &entry);
    }
}