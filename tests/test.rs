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
mod tests {
    use uroboros::{gram, grammar::*, parser::{OrParser, TokenParser}, sym};
    
    use super::*;

    #[test]
    fn remove_left_recursion_0() {
        let _: Grammar<Token> = gram![
            ("expr" => ("term", Token::Equal, "term")),
            ("term" => ("term", Token::Plus, "num") | ("num")),
            ("num" => (Token::VariableLiteral))
        ].remove_left_recursion();
    }
    
    #[test]
    fn token_parser_0() {
        let tokens = vec![ ( Token::Equal, "".to_string() ) ];

        let mut parser = TokenParser::new(Token::Equal);

        assert!(parser.parse( &vec![ ( Token::Equal, "".to_string() ) ] ).is_ok());
    }

    #[test]
    fn token_parser_1() {
        let mut parser = TokenParser::new(Token::Semicolon);

        assert!(parser.parse( &vec![ ( Token::Equal, "".to_string() ) ] ).is_err());
    }

    #[test]
    fn or_parser_0() {
        let mut parser = 
            OrParser::new(
                TokenParser::new(Token::OpenParan), 
                TokenParser::new(Token::ClosenParan)
            );

        assert!(parser.parse( &vec![ ( Token::OpenParan, "".to_string() ) ] ).is_ok());
        assert!(parser.parse( &vec![ ( Token::ClosenParan, "".to_string() ) ] ).is_ok());
        assert!(parser.parse( &vec![ ( Token::Equal, "".to_string() ) ] ).is_err());
    }
}