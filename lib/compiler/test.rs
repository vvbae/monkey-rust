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
    let tests = vec![
        TestCase {
            input: "1+2".to_string(),
            expected_constants: vec![Object::Integer(1), Object::Integer(2)],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpAdd, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "1; 2".to_string(),
            expected_constants: vec![Object::Integer(1), Object::Integer(2)],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpPop, None),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "1-2".to_string(),
            expected_constants: vec![Object::Integer(1), Object::Integer(2)],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpSub, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "1 * 2".to_string(),
            expected_constants: vec![Object::Integer(1), Object::Integer(2)],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpMul, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "2/1".to_string(),
            expected_constants: vec![Object::Integer(2), Object::Integer(1)],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpDiv, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "-1".to_string(),
            expected_constants: vec![],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpMinus, None),
                make(Opcode::OpPop, None),
            ],
        },
    ];

    run_tests(tests)
}

#[test]
fn test_bool_expr() {
    let tests = vec![
        TestCase {
            input: "true".to_string(),
            expected_constants: vec![],
            expected_instructions: vec![make(Opcode::OpTrue, None), make(Opcode::OpPop, None)],
        },
        TestCase {
            input: "false".to_string(),
            expected_constants: vec![],
            expected_instructions: vec![make(Opcode::OpFalse, None), make(Opcode::OpPop, None)],
        },
        TestCase {
            input: "1 > 2".to_string(),
            expected_constants: vec![Object::Integer(1), Object::Integer(2)],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpGreaterThan, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "1 < 2".to_string(),
            expected_constants: vec![Object::Integer(2), Object::Integer(1)],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpGreaterThan, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "1 == 2".to_string(),
            expected_constants: vec![Object::Integer(1), Object::Integer(2)],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpEqual, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "1 != 2".to_string(),
            expected_constants: vec![Object::Integer(1), Object::Integer(2)],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpNotEqual, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "true == false".to_string(),
            expected_constants: vec![],
            expected_instructions: vec![
                make(Opcode::OpTrue, None),
                make(Opcode::OpFalse, None),
                make(Opcode::OpEqual, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "true != false".to_string(),
            expected_constants: vec![],
            expected_instructions: vec![
                make(Opcode::OpTrue, None),
                make(Opcode::OpFalse, None),
                make(Opcode::OpNotEqual, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "!true".to_string(),
            expected_constants: vec![],
            expected_instructions: vec![
                make(Opcode::OpTrue, None),
                make(Opcode::OpBang, None),
                make(Opcode::OpPop, None),
            ],
        },
    ];

    run_tests(tests)
}

#[test]
fn test_conditionals() {
    let tests = vec![
        TestCase {
            input: "if (true) { 10 }; 3333;".to_string(),
            expected_constants: vec![Object::Integer(10), Object::Integer(3333)],
            expected_instructions: vec![
                make(Opcode::OpTrue, None),
                make(Opcode::OpJumpNotTruthy, Some(vec![10])),
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpJump, Some(vec![11])),
                make(Opcode::OpNull, None), // the alternative placeholder
                make(Opcode::OpPop, None),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "if (true) { 10 } else { 20 }; 3333;".to_string(),
            expected_constants: vec![
                Object::Integer(10),
                Object::Integer(20),
                Object::Integer(3333),
            ],
            expected_instructions: vec![
                make(Opcode::OpTrue, None),
                make(Opcode::OpJumpNotTruthy, Some(vec![10])),
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpJump, Some(vec![13])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpPop, None),
                make(Opcode::OpConstant, Some(vec![2])),
                make(Opcode::OpPop, None),
            ],
        },
    ];

    run_tests(tests);
}

#[test]
fn test_global_let_stmts() {
    let tests = vec![
        TestCase {
            input: "let one = 1; let two = 2;".to_string(),
            expected_constants: vec![Object::Integer(1), Object::Integer(2)],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpSetGlobal, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpSetGlobal, Some(vec![1])),
            ],
        },
        TestCase {
            input: "let one = 1; one;".to_string(),
            expected_constants: vec![Object::Integer(1)],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpSetGlobal, Some(vec![0])),
                make(Opcode::OpGetGlobal, Some(vec![0])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "let one = 1; let two = one; two;".to_string(),
            expected_constants: vec![],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpSetGlobal, Some(vec![0])),
                make(Opcode::OpGetGlobal, Some(vec![0])),
                make(Opcode::OpSetGlobal, Some(vec![1])),
                make(Opcode::OpGetGlobal, Some(vec![1])),
                make(Opcode::OpPop, None),
            ],
        },
    ];

    run_tests(tests);
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
        compiler.compile(program);

        let bytecode = compiler.bytecode();

        test_instructions(test.expected_instructions, bytecode.instructions);
        test_constants(test.expected_constants, bytecode.constants);
    }
}
