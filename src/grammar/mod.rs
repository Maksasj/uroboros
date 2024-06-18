mod macros;

#[derive(Debug, PartialEq, Clone)]
pub enum GrammaSymbol<T> {
    NonTerminal(String),
    Terminal(T),
    Sigma
}

impl<T : std::fmt::Debug> GrammaSymbol<T> {
    fn debug_log_grammar_symbol(&self) {
        match self {
            GrammaSymbol::NonTerminal(nonterm) => {
                print!("<{}>", nonterm);
            },
            GrammaSymbol::Terminal(term) => {
                print!("{:?}", term);
            },
            GrammaSymbol::Sigma => {
                print!("ε");
            },
        }
    }

    pub fn to_string(&self) -> String {
        return match self {
            GrammaSymbol::<T>::NonTerminal(nonterm) => format!("<{}>", nonterm),
            GrammaSymbol::<T>::Terminal(term) => format!("{:?}", term),
            GrammaSymbol::<T>::Sigma => String::from("ε"),
        }
    }
}

type ProductionLeft<T> = GrammaSymbol<T>;
type ProductionRight<T> = Vec<Vec<GrammaSymbol<T>>>; 

#[derive(Debug, Clone)]
pub struct Production<T> {
    pub left: ProductionLeft<T>,
    pub right: ProductionRight<T>
}

impl<T> Production<T> where T : PartialEq + Clone {
    fn is_result_left_recursive(&self, result: &Vec<GrammaSymbol<T>>) -> bool {
        if result.len() <= 0 {
            return false;
        }

        if self.left == result[0] {
            return true;
        }

        false
    }

    pub fn is_left_recursive(&self) -> bool {
        if self.right.len() <= 0 {
            return false;
        }

        for right in self.right.iter() {
            if self.is_result_left_recursive(&right) {
                return true
            }
        }
        
        false
    }

    pub fn get_recursive_rights(&self) -> Vec<Vec<GrammaSymbol<T>>> {
        let res: Vec<Vec<GrammaSymbol<T>>> = self.right
            .iter()
            .filter(| &right | self.is_result_left_recursive(right))
            .cloned()
            .collect();
        
        res
    }

    pub fn get_not_recursive_rights(&self) -> Vec<Vec<GrammaSymbol<T>>> {
        let res: Vec<Vec<GrammaSymbol<T>>> = self.right
            .iter()
            .filter(| &right | !self.is_result_left_recursive(right))
            .cloned()
            .collect();
        
        res
    }
}

pub type Grammar<T> = Vec<Production<T>>;

#[allow(dead_code)]
pub trait GrammarTrait<T> {
    fn debug_log(&self);
    fn remove_left_recursion(&self) -> Grammar<T>;
}

impl<T> GrammarTrait<T> for Grammar<T> where T : std::fmt::Debug + PartialEq + Clone {
    fn debug_log(&self) {
        for prod in self.iter() {
            prod.left.debug_log_grammar_symbol();
            print!(" -> ");

            for (i, right) in prod.right.iter().enumerate() {
                for sym in right.iter() {
                    sym.debug_log_grammar_symbol();
                    print!(" ");
                }
                
                if i != prod.right.len() - 1 {
                    print!("| ");
                }
            }

            print!("\n");
        }
    }

    // Todo check for depth recursion
    fn remove_left_recursion(&self) -> Grammar<T> {
        let mut new: Vec<Production<T>> = Grammar::<T>::new();

        for prod in self.iter() {
            if !prod.is_left_recursive() {
                new.push(prod.clone());
                continue;
            }

            let a_left: GrammaSymbol::<T> = prod.left.clone();
            let a_star_left: GrammaSymbol::<T> = GrammaSymbol::<T>::NonTerminal(format!("{}'", prod.left.to_string()));

            let a_right: Vec<Vec<GrammaSymbol<T>>> = prod
                .get_not_recursive_rights()
                .iter_mut()
                .map(| right | {
                    right.push(a_star_left.clone());
                    right.clone()
                })
                .collect();

            let mut a_star_right: Vec<Vec<GrammaSymbol<T>>> = prod
                .get_recursive_rights()
                .iter_mut()
                .map(| right | {
                    right.push(a_star_left.clone());
                    Vec::from(&right[1..right.len()])
                })
                .collect();

            a_star_right.push(vec! [ GrammaSymbol::Sigma ]);

            new.push(Production {
                left: a_left,
                right: a_right
            });

            new.push(Production {
                left: a_star_left,
                right: a_star_right
            });
        }

        new
    }
}
