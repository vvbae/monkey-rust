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
        make_testcase("1 - 2", Object::Integer(-1)),
        make_testcase("1 * 2", Object::Integer(2)),
        make_testcase("4 / 2", Object::Integer(2)),
        make_testcase("50 / 2 * 2 + 10 - 5", Object::Integer(55)),
        make_testcase("5 * (2 + 10)", Object::Integer(60)),
        make_testcase("5 + 5 + 5 + 5 - 10", Object::Integer(10)),
        make_testcase("2 * 2 * 2 * 2 * 2", Object::Integer(32)),
        make_testcase("5 * 2 + 10", Object::Integer(20)),
        make_testcase("5 + 2 * 10", Object::Integer(25)),
        make_testcase("5 * (2 + 10)", Object::Integer(60)),
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
        compiler.compile(program);

        let bytecode = compiler.bytecode();

        let mut vm = VM::new(bytecode);
        vm.run().unwrap();

        let stack_ele = vm.last_popped_stack_ele();

        test_expected(test.expected, &stack_ele);
    }
}

fn test_expected(expected: Object, actual: &Object) {
    match expected {
        Object::Integer(v) => test_int_obj(v, actual.clone()),
        _ => todo!(),
    }
}
