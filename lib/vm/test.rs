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

#[test]
fn test_bool_expr() {
    let tests = vec![
        make_testcase("true", Object::Boolean(true)),
        make_testcase("false", Object::Boolean(false)),
        make_testcase("1 < 2", Object::Boolean(true)),
        make_testcase("1 > 2", Object::Boolean(false)),
        make_testcase("1 < 1", Object::Boolean(false)),
        make_testcase("1 > 1", Object::Boolean(false)),
        make_testcase("1 == 1", Object::Boolean(true)),
        make_testcase("1 != 1", Object::Boolean(false)),
        make_testcase("1 == 2", Object::Boolean(false)),
        make_testcase("true == true", Object::Boolean(true)),
        make_testcase("false == false", Object::Boolean(true)),
        make_testcase("true == false", Object::Boolean(false)),
        make_testcase("true != false", Object::Boolean(true)),
        make_testcase("false != true", Object::Boolean(true)),
        make_testcase("(1 < 2) == true", Object::Boolean(true)),
        make_testcase("(1 < 2) == false", Object::Boolean(false)),
        make_testcase("(1 > 2) == true", Object::Boolean(false)),
        make_testcase("(1 > 2) == false", Object::Boolean(true)),
    ];

    run_tests(tests);
}

fn test_int_obj(expected: i64, actual: Object) {
    match actual {
        Object::Integer(v) => assert_eq!(
            expected, v,
            "object has wrong value. got={:?}, want={:?}",
            v, expected
        ),
        _ => panic!("object is not Integer. got={:?}", actual),
    }
}

fn test_bool_obj(expected: bool, actual: Object) {
    match actual {
        Object::Boolean(v) => assert_eq!(
            expected, v,
            "object has wrong value. got={:?}, want={:?}",
            v, expected
        ),
        _ => panic!("object is not Boolean. got={:?}", actual),
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
        Object::Boolean(v) => test_bool_obj(v, actual.clone()),
        _ => todo!(),
    }
}
