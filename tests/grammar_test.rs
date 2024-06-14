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
    use uroboros::{grammar::*, prod, sym};
    
    use super::*;

    #[test]
    fn it_works() {
        let grammar: Grammar<TokenType> = vec![
            prod!("expr" => ("term", TokenType::Equal, "term")),
            prod!("term" => ("term", TokenType::Plus, "num") | ("num")),
            prod!("num" => (TokenType::VariableLiteral)),
        ].remove_left_recursion();
    }
}