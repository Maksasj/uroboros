#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    VariableLiteral,
    
    Plus,
    Equal,
    
    OpenParan,
    ClosenParan,
    Semicolon
}

#[cfg(test)]
mod grammar {    
    use super::*;

    #[test]
    fn production_0() {
        use uroboros::{grammar::*, prod, sym};
        
        {
            let prod: Production<Token> = prod!("term" => ("term"));
            assert_eq!(true, prod.is_left_recursive());
        }
        
        assert_eq!(true, prod!("term" => ("term", Token::Plus)).is_left_recursive());
        assert_eq!(true, prod!("term" => ("term", Token::Plus, "term")).is_left_recursive());

        {
            let prod: Production<Token> = prod!("term" => ("expr"));
            assert_eq!(false, prod.is_left_recursive());
        }

        {
            let prod: Production<Token> =  prod!("term" => ("expr", "term"));
            assert_eq!(false, prod.is_left_recursive());
        }

        assert_eq!(false, prod!("term" => ("expr", Token::Plus)).is_left_recursive());
        assert_eq!(false, prod!("term" => (Token::Plus)).is_left_recursive());
    }

    #[test]
    fn grammar_0() {
        use uroboros::{gram, sym, grammar::*};

        let _: Grammar<Token> = gram![
            ("expr" => ("term", Token::Equal, "term")),
            ("term" => ("term", Token::Plus, "num") | ("num")),
            ("num" => (Token::VariableLiteral))
        ].remove_left_recursion();
    }

    // Todo, more grammar tests
}