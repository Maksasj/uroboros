#[derive(Debug, Copy, Clone, PartialEq)]
enum TokenType {
    VariableLiteral,
    
    Plus,
    Equal,
    
    OpenParan,
    ClosenParan,
    Semicolon
}

#[cfg(test)]
mod tests {
    use uroboros::{grammar::*, gram, sym};
    
    use super::*;

    #[test]
    fn it_works() {
        let grammar: Grammar<TokenType> = gram![
            ("expr" => ("term", TokenType::Equal, "term")),
            ("term" => ("term", TokenType::Plus, "num") | ("num")),
            ("num" => (TokenType::VariableLiteral))
        ].remove_left_recursion();
    }
}