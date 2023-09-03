use crate::{
    code::{make, Opcode},
    common::parse,
};

use super::*;

struct TestCase {
    pub input: String,
    pub expected_constants: Vec<Object>,
    pub expected_instructions: Vec<Instructions>,
}

#[test]
fn test_int_arithmetic() {
    let tests = vec![TestCase {
        input: "1+2".to_string(),
        expected_constants: vec![Object::Integer(1), Object::Integer(2)],
        expected_instructions: vec![
            make(Opcode::OpConstant, vec![0]),
            make(Opcode::OpConstant, vec![1]),
        ],
    }];

    run_tests(tests)
}

fn test_int_object(expected: i64, actual: Object) {
    match actual {
        Object::Integer(v) => assert_eq!(expected, v),
        _ => panic!("object is not Integer. got={:?}", actual),
    }
}

fn test_instructions(expected: Vec<Instructions>, actual: Instructions) {
    let concatted = expected.concat();

    assert_eq!(concatted, actual);
}

fn test_constants(expected: Vec<Object>, actual: Vec<Object>) {
    for (i, constant) in expected.iter().enumerate() {
        match constant {
            Object::Integer(v) => test_int_object(*v, actual[i].clone()),
            Object::Boolean(_) => todo!(),
            Object::String(_) => todo!(),
            Object::Array(_) => todo!(),
            Object::Hash(_) => todo!(),
            Object::Function(_, _, _) => todo!(),
            Object::Builtin(_, _, _) => todo!(),
            Object::Null => todo!(),
            Object::ReturnValue(_) => todo!(),
            Object::Error(_) => todo!(),
        }
    }
}

fn run_tests(tests: Vec<TestCase>) {
    for test in tests {
        let program = parse(test.input);
        let mut compiler = Compiler::new();
        let res = compiler.compile(program);

        let bytecode = compiler.bytecode();

        test_instructions(test.expected_instructions, bytecode.instructions);
        test_constants(test.expected_constants, bytecode.constants);
    }
}
