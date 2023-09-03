use crate::{
    lexer::{token::Tokens, Lexer},
    parser::{ast::Program, Parser},
};

pub fn parse(input: String) -> Program {
    let (_, l) = Lexer::lex_tokens(input.as_bytes()).unwrap();
    let tokens = Tokens::new(&l);
    let (_, p) = Parser::parse_tokens(tokens).unwrap();

    p
}
