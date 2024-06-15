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
mod parser {
    use uroboros::{gram, grammar::*, parser::{llgparser::LLGParser, operator::{Exact, Many, Or, SeqOf}, Parser}, sym};
    
    use super::*;

    #[test]
    fn llgparser_0() {
        let grammar: Grammar<Token> = gram![
            ("expr" => ("term", Token::Equal, "term")),
            ("term" => ("term", Token::Plus, "num") | ("num")),
            ("num" => (Token::VariableLiteral))
        ].remove_left_recursion();

        let parser = LLGParser::new(grammar, sym!("expr"));

        assert!(parser.parse( &vec![ Token::VariableLiteral, Token::Equal, Token::VariableLiteral, Token::Semicolon ] ).is_ok());
        assert!(parser.parse( &vec![ Token::VariableLiteral, Token::Equal, Token::VariableLiteral ] ).is_ok());
        assert!(parser.parse( &vec![ Token::VariableLiteral, Token::Plus, Token::VariableLiteral, Token::Equal, Token::VariableLiteral, Token::Semicolon ] ).is_ok());

        assert!(parser.parse( &vec![ Token::VariableLiteral ] ).is_err());
        assert!(parser.parse( &vec![ Token::VariableLiteral, Token::Equal] ).is_err());
    }
    
    #[test]
    fn exact_parser_0() {
        let parser = Exact::new(Token::Equal);

        assert!(parser.parse( &vec![ Token::Equal ] ).is_ok());
        assert!(parser.parse( &vec![ Token::Equal, Token::Semicolon ] ).is_ok());
        assert!(parser.parse( &vec![ Token::Semicolon ] ).is_err());
    }

    #[test]
    fn or_parser_0() {
        let parser = 
            Or::new(
            Exact::new(Token::OpenParan), 
            Exact::new(Token::ClosenParan)
            );

        assert_eq!(true, parser.parse( &vec![ Token::OpenParan ] ).is_ok());
        assert_eq!(true, parser.parse( &vec![ Token::ClosenParan ] ).is_ok());
        assert_eq!(false, parser.parse( &vec![ Token::Equal ] ).is_ok());
    }

    #[test]
    fn many_parser_0() {
        let parser = Many::new(Exact::new(Token::OpenParan)); 

        // Todo, wont work since for now child parsers do not consume tokens

        // assert_eq!(true, parser.parse( &vec![ Token::OpenParan ] ).is_ok());
        // assert_eq!(true, parser.parse( &vec![ Token::OpenParan ] ).is_ok());
        assert_eq!(true, parser.parse( &vec![ Token::Equal ] ).is_err());
        assert_eq!(true, parser.parse( &vec![ ] ).is_err());
    }

    #[test]
    fn seqof_parser_0() {
        let parser = SeqOf::new(vec![ Exact::new(Token::OpenParan) ]); 

        // Todo, wont work since for now child parsers do not consume tokens
        assert_eq!(true, parser.parse( &vec![ Token::OpenParan, Token::OpenParan, Token::OpenParan ] ).is_ok());
        assert_eq!(true, parser.parse( &vec![ Token::OpenParan ] ).is_ok());
        assert_eq!(true, parser.parse( &vec![ Token::OpenParan ] ).is_ok());
        assert_eq!(true, parser.parse( &vec![ Token::Equal ] ).is_err());
        assert_eq!(true, parser.parse( &vec![ ] ).is_err());
    }
}