mod macros;

#[derive(Debug, PartialEq, Clone)]
pub enum GrammaSymbols<T> {
    NonTerminal(String),
    Terminal(T),
    Sigma
}

impl<T : std::fmt::Debug> GrammaSymbols<T> {
    fn debug_log_grammar_symbol(&self) {
        match self {
            GrammaSymbols::NonTerminal(nonterm) => {
                print!("<{}>", nonterm);
            },
            GrammaSymbols::Terminal(term) => {
                print!("{:?}", term);
            },
            GrammaSymbols::Sigma => {
                print!("ϵ");
            },
        }
    }

    pub fn to_string(&self) -> String {
        return match self {
            GrammaSymbols::<T>::NonTerminal(nonterm) => format!("<{}>", nonterm),
            GrammaSymbols::<T>::Terminal(term) => format!("{:?}", term),
            GrammaSymbols::<T>::Sigma => String::from("ϵ"),
        }
    }
}

type ProductionLeft<T> = GrammaSymbols<T>;
type ProductionRight<T> = Vec<Vec<GrammaSymbols<T>>>; 

#[derive(Debug, Clone)]
pub struct Production<T> {
    pub left: ProductionLeft<T>,
    pub right: ProductionRight<T>
}

impl<T> Production<T> where T : PartialEq + Clone {
    fn is_result_left_recursive(&self, result: &Vec<GrammaSymbols<T>>) -> bool {
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

    pub fn get_recursive_rights(&self) -> Vec<Vec<GrammaSymbols<T>>> {
        let res: Vec<Vec<GrammaSymbols<T>>> = self.right
            .iter()
            .filter(| &right | self.is_result_left_recursive(right))
            .cloned()
            .collect();
        
        res
    }

    pub fn get_not_recursive_rights(&self) -> Vec<Vec<GrammaSymbols<T>>> {
        let res: Vec<Vec<GrammaSymbols<T>>> = self.right
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

            let a_left: GrammaSymbols::<T> = prod.left.clone();
            let a_star_left: GrammaSymbols::<T> = GrammaSymbols::<T>::NonTerminal(format!("{}'", prod.left.to_string()));

            let a_right: Vec<Vec<GrammaSymbols<T>>> = prod
                .get_not_recursive_rights()
                .iter_mut()
                .map(| right | {
                    right.push(a_star_left.clone());
                    right.clone()
                })
                .collect();

            let mut a_star_right: Vec<Vec<GrammaSymbols<T>>> = prod
                .get_recursive_rights()
                .iter_mut()
                .map(| right | {
                    right.push(a_star_left.clone());
                    Vec::from(&right[1..right.len()])
                })
                .collect();

            a_star_right.push(vec! [ GrammaSymbols::Sigma ]);

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
