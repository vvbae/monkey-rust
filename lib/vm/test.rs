use crate::{common::parse, compiler::Compiler, evaluator::object::Object};

use super::*;

struct TestCase {
    pub input: String,
    pub expected: Object,
}

fn make_testcase(input: &str, expected: Object) -> TestCase {
    TestCase {
        input: input.to_string(),
        expected,
    }
}

#[test]
fn test_int_arithmetic() {
    let tests = vec![
        make_testcase("1", Object::Integer(1)),
        make_testcase("2", Object::Integer(2)),
        make_testcase("1 + 2", Object::Integer(3)),
    ];

    run_tests(tests)
}

fn test_int_obj(expected: i64, actual: Object) {
    match actual {
        Object::Integer(v) => assert_eq!(expected, v),
        _ => panic!("object is not Integer. got={:?}", actual),
    }
}

fn run_tests(tests: Vec<TestCase>) {
    for test in tests {
        let program = parse(test.input);
        let mut compiler = Compiler::new();
        let res = compiler.compile(program);

        let bytecode = compiler.bytecode();
        // println!("{:?}", bytecode.instructions);
        // println!("{:?}", bytecode.constants);

        let mut vm = VM::new(bytecode);
        vm.run().unwrap();

        let stack_ele = vm.stack_top().unwrap();

        test_expected(test.expected, stack_ele);
    }
}

fn test_expected(expected: Object, actual: &Object) {
    match expected {
        Object::Integer(v) => test_int_obj(v, actual.clone()),
        _ => todo!(),
    }
}
