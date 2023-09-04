use std::{collections::HashMap, hash};

use crate::{
    common::{oth, parse},
    compiler::Compiler,
    evaluator::object::Object,
};

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
        make_testcase("-5", Object::Integer(-5)),
        make_testcase("-10", Object::Integer(-10)),
        make_testcase("-50 + 100 + -50", Object::Integer(0)),
        make_testcase("(5 + 10 * 2 + 15 / 3) * 2 + -10", Object::Integer(50)),
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
        make_testcase("!true", Object::Boolean(false)),
        make_testcase("!false", Object::Boolean(true)),
        make_testcase("!5", Object::Boolean(false)),
        make_testcase("!!true", Object::Boolean(true)),
        make_testcase("!!false", Object::Boolean(false)),
        make_testcase("!!5", Object::Boolean(true)),
        make_testcase("!(if (false) { 5; })", Object::Boolean(true)),
    ];

    run_tests(tests);
}

#[test]
fn test_conditionals() {
    let tests = vec![
        make_testcase("if (true) { 10 }", Object::Integer(10)),
        make_testcase("if (true) { 10 } else { 20 }", Object::Integer(10)),
        make_testcase("if (false) { 10 } else { 20 } ", Object::Integer(20)),
        make_testcase("if (1) { 10 }", Object::Integer(10)),
        make_testcase("if (1 < 2) { 10 }", Object::Integer(10)),
        make_testcase("if (1 < 2) { 10 } else { 20 }", Object::Integer(10)),
        make_testcase("if (1 > 2) { 10 } else { 20 }", Object::Integer(20)),
        make_testcase("if (1 > 2) { 10 }", NULL),
        make_testcase("if (false) { 10 }", NULL),
        make_testcase(
            "if ((if (false) { 10 })) { 10 } else { 20 }",
            Object::Integer(20),
        ),
    ];

    run_tests(tests);
}

#[test]
fn test_global_let_stmts() {
    let tests = vec![
        make_testcase("let one = 1; one", Object::Integer(1)),
        make_testcase("let one = 1; let two = 2; one + two", Object::Integer(3)),
        make_testcase(
            "let one = 1; let two = one + one; one + two",
            Object::Integer(3),
        ),
    ];

    run_tests(tests);
}

#[test]
fn test_string_expr() {
    let tests = vec![
        make_testcase("\"monkey\"", Object::String("monkey".to_string())),
        make_testcase("\"mon\" + \"key\"", Object::String("monkey".to_string())),
        make_testcase(
            "\"mon\" + \"key\" + \"banana\"",
            Object::String("monkeybanana".to_string()),
        ),
    ];

    run_tests(tests);
}

#[test]
fn test_array_literals() {
    let tests = vec![
        make_testcase("[]", Object::Array(vec![])),
        make_testcase(
            "[1, 2, 3]",
            Object::Array(vec![
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3),
            ]),
        ),
        make_testcase(
            "[1 + 2, 3 * 4, 5 + 6]",
            Object::Array(vec![
                Object::Integer(3),
                Object::Integer(12),
                Object::Integer(11),
            ]),
        ),
    ];

    run_tests(tests);
}

#[test]
fn test_hash_literals() {
    let tests = vec![
        make_testcase("{}", Object::Hash(HashMap::new())),
        make_testcase(
            "{1: 2, 2: 3}",
            Object::Hash(HashMap::from([
                (oth(Object::Integer(1)), Object::Integer(2)),
                (oth(Object::Integer(2)), Object::Integer(3)),
            ])),
        ),
        // parsing error
        // make_testcase(
        //     "{1 + 1: 2 * 2, 3 + 3: 4 * 4}",
        //     Object::Hash(HashMap::from([
        //         (oth(Object::Integer(2)), Object::Integer(4)),
        //         (oth(Object::Integer(6)), Object::Integer(16)),
        //     ])),
        // ),
    ];

    run_tests(tests);
}

#[test]
fn test_index_expr() {
    let tests = vec![
        make_testcase("[1, 2, 3][1]", Object::Integer(2)),
        make_testcase("[1, 2, 3][0 + 2]", Object::Integer(3)),
        make_testcase("[[1, 1, 1]][0][0]", Object::Integer(1)),
        make_testcase("[][0]", Object::Null),
        make_testcase("[1, 2, 3][99]", Object::Null),
        make_testcase("[1][-1]", Object::Null),
        make_testcase("{1: 1, 2: 2}[1]", Object::Integer(1)),
        make_testcase("{1: 1, 2: 2}[2]", Object::Integer(2)),
        make_testcase("{1: 1}[0]", Object::Null),
        make_testcase("{}[0]", Object::Null),
    ];

    run_tests(tests);
}

fn test_int_obj(expected: i64, actual: Object) {
    match actual {
        Object::Integer(v) => {
            assert_eq!(
                expected, v,
                "object has wrong value. got={:?}, want={:?}",
                v, expected
            )
        }
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

fn test_string_obj(expected: String, actual: Object) {
    match actual {
        Object::String(v) => assert_eq!(
            expected, v,
            "object has wrong value. got={:?}, want={:?}",
            v, expected
        ),
        _ => panic!("object is not String. got={:?}", actual),
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
        Object::String(v) => test_string_obj(v, actual.clone()),
        Object::Array(arr) => {
            let result = match actual {
                Object::Array(v) => v.clone(),
                _ => unimplemented!(),
            };

            assert_eq!(arr.len(), result.len());
            assert_eq!(arr, result);
        }
        Object::Hash(hashmap) => {
            println!("{:?}", hashmap);
            let expected_pairs = hashmap.into_iter().collect::<Vec<_>>();
            match actual {
                Object::Hash(map) => {
                    for (key, val) in expected_pairs {
                        assert_eq!(val, map[&key])
                    }
                }
                _ => unimplemented!(),
            }
        }
        Object::Null => match actual {
            Object::Null => {}
            _ => panic!("object is not Null: {:?} ({:?})", actual, actual),
        },
        _ => todo!(),
    }
}
