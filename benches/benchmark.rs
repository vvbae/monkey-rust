use criterion::{criterion_group, criterion_main, Criterion};
use monkey_lib::{
    compiler::Compiler,
    lexer::{token::Tokens, Lexer},
    parser::Parser,
    vm::VM,
};

fn compile() {
    let input = "
    let fibonacci = fn(x) {
        if (x == 0) { 0
          } else {
            if (x == 1) {
              return 1;
            } else {
              fibonacci(x - 1) + fibonacci(x - 2);
            }
        } };
        fibonacci(35);
    "
    .to_string();

    let mut compiler = Compiler::new();
    let lex_tokens = Lexer::lex_tokens(input.as_bytes());
    match lex_tokens {
        Ok((_, r)) => {
            let tokens = Tokens::new(&r);
            let parsed = Parser::parse_tokens(tokens);
            match parsed {
                Ok((_, program)) => {
                    compiler.compile(program);
                    let mut machine = VM::new(compiler.bytecode());
                    machine.run().unwrap();

                    let last_popped = machine.last_popped_stack_ele();
                    println!("{}", last_popped);
                }
                Err(_) => todo!(),
            }
        }
        Err(_) => todo!(),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("compile", |b| b.iter(|| compile()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
