use crate::{
    evaluator::object::Object,
    lexer::{token::Tokens, Lexer},
    parser::{ast::Program, Parser},
};

pub fn parse(input: String) -> Program {
    let (_, l) = Lexer::lex_tokens(input.as_bytes()).unwrap();
    let tokens = Tokens::new(&l);
    let (_, p) = Parser::parse_tokens(tokens).unwrap();

    p
}

pub fn oth(object: Object) -> Object {
    match object {
        Object::Integer(i) => Object::Integer(i),
        Object::Boolean(b) => Object::Boolean(b),
        Object::String(s) => Object::String(s),
        Object::Error(s) => Object::Error(s),
        x => Object::Error(format!("{} is not hashable", x)),
    }
}
