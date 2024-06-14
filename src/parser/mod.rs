use std::result;

use crate::{grammar::{GrammaSymbols, Production}, sigma};

struct Parser {
    head: usize
}

type Result<T> = result::Result<T, ()>;

impl Parser {
    fn new() -> Self {
        Parser {
            head: 0
        }
    }

    fn parse<T>(&mut self, tokens: &Vec<(T, String)>, grammar: &Vec<Production<T>>, entry: &GrammaSymbols<T>) -> Result<()> where T : PartialEq + Clone {
        match entry {
            GrammaSymbols::<T>::NonTerminal(_) => {
                for prod in grammar.iter() {
                    if prod.left != entry.clone() { continue; }

                    let backup: usize = self.head;

                    let mut suc: bool = false;

                    for right in prod.right.iter() {
                        for symbol in right.iter() {
                            if self.parse(tokens, grammar, &symbol.clone()).is_err() {
                                self.head = backup;
                                suc = false;
                                break;
                            } else {
                                suc = true;
                            }
                        }
                    }

                    return match suc {
                        true => Ok(()),
                        false => Err(()),
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
                    return Ok(());
                }
        
                return Err(()); 
            },
            sigma!() => {
                return Ok(());
            }
        }
    }
}