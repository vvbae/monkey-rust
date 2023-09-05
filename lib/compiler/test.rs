use crate::{
    code::{make, Opcode},
    common::parse,
};

use super::*;

enum Constant {
    Object(Object),
    Instructions(Vec<Instructions>),
}

struct TestCase {
    pub input: String,
    pub expected_constants: Vec<Constant>,
    pub expected_instructions: Vec<Instructions>,
}

#[test]
fn test_int_arithmetic() {
    let tests = vec![
        TestCase {
            input: "1+2".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(2)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpAdd, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "1; 2".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(2)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpPop, None),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "1-2".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(2)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpSub, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "1 * 2".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(2)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpMul, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "2/1".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(2)),
                Constant::Object(Object::Integer(1)),
            ],
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
            expected_constants: vec![
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(2)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpGreaterThan, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "1 < 2".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(2)),
                Constant::Object(Object::Integer(1)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpGreaterThan, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "1 == 2".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(2)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpEqual, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "1 != 2".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(2)),
            ],
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
            expected_constants: vec![
                Constant::Object(Object::Integer(10)),
                Constant::Object(Object::Integer(3333)),
            ],
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
                Constant::Object(Object::Integer(10)),
                Constant::Object(Object::Integer(20)),
                Constant::Object(Object::Integer(3333)),
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
            expected_constants: vec![
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(2)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpSetGlobal, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpSetGlobal, Some(vec![1])),
            ],
        },
        TestCase {
            input: "let one = 1; one;".to_string(),
            expected_constants: vec![Constant::Object(Object::Integer(1))],
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

#[test]
fn test_string_expr() {
    let tests = vec![
        TestCase {
            input: "\"monkey\"".to_string(),
            expected_constants: vec![Constant::Object(Object::String("monkey".to_string()))],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: " \"mon\" + \"key\" ".to_string(),
            expected_constants: vec![
                Constant::Object(Object::String("mon".to_string())),
                Constant::Object(Object::String("key".to_string())),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpAdd, None),
                make(Opcode::OpPop, None),
            ],
        },
    ];

    run_tests(tests);
}

#[test]
fn test_array_literals() {
    let tests = vec![
        TestCase {
            input: "[]".to_string(),
            expected_constants: vec![],
            expected_instructions: vec![
                make(Opcode::OpArray, Some(vec![0])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "[1, 2, 3]".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(2)),
                Constant::Object(Object::Integer(3)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpConstant, Some(vec![2])),
                make(Opcode::OpArray, Some(vec![3])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "[1 + 2, 3 - 4, 5 * 6]".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(2)),
                Constant::Object(Object::Integer(3)),
                Constant::Object(Object::Integer(4)),
                Constant::Object(Object::Integer(5)),
                Constant::Object(Object::Integer(6)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpAdd, None),
                make(Opcode::OpConstant, Some(vec![2])),
                make(Opcode::OpConstant, Some(vec![3])),
                make(Opcode::OpSub, None),
                make(Opcode::OpConstant, Some(vec![4])),
                make(Opcode::OpConstant, Some(vec![5])),
                make(Opcode::OpMul, None),
                make(Opcode::OpArray, Some(vec![3])),
                make(Opcode::OpPop, None),
            ],
        },
    ];

    run_tests(tests);
}

#[test]
fn test_hash_literals() {
    let tests = vec![
        TestCase {
            input: "{}".to_string(),
            expected_constants: vec![],
            expected_instructions: vec![
                make(Opcode::OpHash, Some(vec![0])),
                make(Opcode::OpPop, None),
            ],
        },
        // TestCase {
        //     input: "{1: 2, 3: 4, 5: 6}".to_string(),
        //     expected_constants: vec![
        //         Object::Integer(1),
        //         Object::Integer(2),
        //         Object::Integer(3),
        //         Object::Integer(4),
        //         Object::Integer(5),
        //         Object::Integer(6),
        //     ],
        //     expected_instructions: vec![
        //         make(Opcode::OpConstant, Some(vec![0])),
        //         make(Opcode::OpConstant, Some(vec![1])),
        //         make(Opcode::OpConstant, Some(vec![2])),
        //         make(Opcode::OpConstant, Some(vec![3])),
        //         make(Opcode::OpConstant, Some(vec![4])),
        //         make(Opcode::OpConstant, Some(vec![5])),
        //         make(Opcode::OpHash, Some(vec![6])),
        //         make(Opcode::OpPop, None),
        //     ],
        // },
        // TestCase {
        //     input: "{1: 2 + 3, 4: 5 * 6}".to_string(),
        //     expected_constants: vec![
        //         Object::Integer(1),
        //         Object::Integer(2),
        //         Object::Integer(3),
        //         Object::Integer(4),
        //         Object::Integer(5),
        //         Object::Integer(6),
        //     ],
        //     expected_instructions: vec![
        //         make(Opcode::OpConstant, Some(vec![0])),
        //         make(Opcode::OpConstant, Some(vec![1])),
        //         make(Opcode::OpConstant, Some(vec![2])),
        //         make(Opcode::OpAdd, None),
        //         make(Opcode::OpConstant, Some(vec![3])),
        //         make(Opcode::OpConstant, Some(vec![4])),
        //         make(Opcode::OpConstant, Some(vec![5])),
        //         make(Opcode::OpMul, None),
        //         make(Opcode::OpHash, Some(vec![4])),
        //         make(Opcode::OpPop, None),
        //     ],
        // },
    ];

    run_tests(tests);
}

#[test]
fn test_index_expr() {
    let tests = vec![
        TestCase {
            input: "[1, 2, 3][1 + 1]".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(2)),
                Constant::Object(Object::Integer(3)),
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(1)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpConstant, Some(vec![2])),
                make(Opcode::OpArray, Some(vec![3])),
                make(Opcode::OpConstant, Some(vec![3])),
                make(Opcode::OpConstant, Some(vec![4])),
                make(Opcode::OpAdd, None),
                make(Opcode::OpIndex, None),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "{1: 2}[2 - 1]".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(2)),
                Constant::Object(Object::Integer(2)),
                Constant::Object(Object::Integer(1)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpHash, Some(vec![2])),
                make(Opcode::OpConstant, Some(vec![2])),
                make(Opcode::OpConstant, Some(vec![3])),
                make(Opcode::OpSub, None),
                make(Opcode::OpIndex, None),
                make(Opcode::OpPop, None),
            ],
        },
    ];

    run_tests(tests);
}

#[test]
fn test_functions() {
    let tests = vec![
        TestCase {
            input: "fn() { return 5 + 10 }".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(5)),
                Constant::Object(Object::Integer(10)),
                Constant::Instructions(vec![
                    make(Opcode::OpConstant, Some(vec![0])),
                    make(Opcode::OpConstant, Some(vec![1])),
                    make(Opcode::OpAdd, None),
                    make(Opcode::OpReturnValue, None),
                ]),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![2])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "fn() { 5 + 10 }".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(5)),
                Constant::Object(Object::Integer(10)),
                Constant::Instructions(vec![
                    make(Opcode::OpConstant, Some(vec![0])),
                    make(Opcode::OpConstant, Some(vec![1])),
                    make(Opcode::OpAdd, None),
                    make(Opcode::OpReturnValue, None),
                ]),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![2])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "fn() { 1; 2 }".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(1)),
                Constant::Object(Object::Integer(2)),
                Constant::Instructions(vec![
                    make(Opcode::OpConstant, Some(vec![0])),
                    make(Opcode::OpPop, None),
                    make(Opcode::OpConstant, Some(vec![1])),
                    make(Opcode::OpReturnValue, None),
                ]),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![2])),
                make(Opcode::OpPop, None),
            ],
        },
    ];

    run_tests(tests);
}

#[test]
fn test_functions_without_return_values() {
    let tests = vec![TestCase {
        input: "fn() { }".to_string(),
        expected_constants: vec![Constant::Instructions(vec![make(Opcode::OpReturn, None)])],
        expected_instructions: vec![
            make(Opcode::OpConstant, Some(vec![0])),
            make(Opcode::OpPop, None),
        ],
    }];

    run_tests(tests);
}

#[test]
fn test_function_calls() {
    let tests = vec![
        TestCase {
            input: "fn() { 24 }();".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(24)),
                Constant::Instructions(vec![
                    make(Opcode::OpConstant, Some(vec![0])),
                    make(Opcode::OpReturnValue, None),
                ]),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpCall, Some(vec![0])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "let noArg = fn() { 24 }; noArg();".to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(24)),
                Constant::Instructions(vec![
                    make(Opcode::OpConstant, Some(vec![0])),
                    make(Opcode::OpReturnValue, None),
                ]),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpSetGlobal, Some(vec![0])),
                make(Opcode::OpGetGlobal, Some(vec![0])),
                make(Opcode::OpCall, Some(vec![0])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "let oneArg = fn(a) { }; oneArg(24);".to_string(),
            expected_constants: vec![
                Constant::Instructions(vec![make(Opcode::OpReturn, None)]),
                Constant::Object(Object::Integer(24)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpSetGlobal, Some(vec![0])),
                make(Opcode::OpGetGlobal, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpCall, Some(vec![1])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "let manyArg = fn(a, b, c) { }; manyArg(24, 25, 26);".to_string(),
            expected_constants: vec![
                Constant::Instructions(vec![make(Opcode::OpReturn, None)]),
                Constant::Object(Object::Integer(24)),
                Constant::Object(Object::Integer(25)),
                Constant::Object(Object::Integer(26)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpSetGlobal, Some(vec![0])),
                make(Opcode::OpGetGlobal, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpConstant, Some(vec![2])),
                make(Opcode::OpConstant, Some(vec![3])),
                make(Opcode::OpCall, Some(vec![3])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "let oneArg = fn(a) { a }; oneArg(24);".to_string(),
            expected_constants: vec![
                Constant::Instructions(vec![
                    make(Opcode::OpGetLocal, Some(vec![0])),
                    make(Opcode::OpReturnValue, None),
                ]),
                Constant::Object(Object::Integer(24)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpSetGlobal, Some(vec![0])),
                make(Opcode::OpGetGlobal, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpCall, Some(vec![1])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "let manyArg = fn(a, b, c) { a; b; c }; manyArg(24, 25, 26);".to_string(),
            expected_constants: vec![
                Constant::Instructions(vec![
                    make(Opcode::OpGetLocal, Some(vec![0])),
                    make(Opcode::OpPop, None),
                    make(Opcode::OpGetLocal, Some(vec![1])),
                    make(Opcode::OpPop, None),
                    make(Opcode::OpGetLocal, Some(vec![2])),
                    make(Opcode::OpReturnValue, None),
                ]),
                Constant::Object(Object::Integer(24)),
                Constant::Object(Object::Integer(25)),
                Constant::Object(Object::Integer(26)),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpSetGlobal, Some(vec![0])),
                make(Opcode::OpGetGlobal, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpConstant, Some(vec![2])),
                make(Opcode::OpConstant, Some(vec![3])),
                make(Opcode::OpCall, Some(vec![3])),
                make(Opcode::OpPop, None),
            ],
        },
    ];

    run_tests(tests);
}

#[test]
fn test_compilation_scopes() {
    let mut compiler = Compiler::new();
    assert_eq!(compiler.scope_index, 0);

    compiler.emit(Opcode::OpMul, None);

    compiler.enter_scope();
    assert_eq!(compiler.scope_index, 1);

    compiler.emit(Opcode::OpSub, None);

    assert_eq!(compiler.scopes[compiler.scope_index].instructions.len(), 1);

    let last_ins = compiler.scopes[compiler.scope_index].last_ins;
    assert_eq!(last_ins.unwrap().opcode, Opcode::OpSub);

    compiler.leave_scope();
    assert_eq!(compiler.scope_index, 0);

    compiler.emit(Opcode::OpAdd, None);

    assert_eq!(compiler.scopes[compiler.scope_index].instructions.len(), 2);
    let last_ins = compiler.scopes[compiler.scope_index].last_ins;
    assert_eq!(last_ins.unwrap().opcode, Opcode::OpAdd);

    let prev_ins = compiler.scopes[compiler.scope_index].prev_ins;
    assert_eq!(prev_ins.unwrap().opcode, Opcode::OpMul);
}

#[test]
fn test_let_stmt_scopes() {
    let tests = vec![
        TestCase {
            input: "let num = 55;
                    fn() {
                        num
                    }"
            .to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(55)),
                Constant::Instructions(vec![
                    make(Opcode::OpGetGlobal, Some(vec![0])),
                    make(Opcode::OpReturnValue, None),
                ]),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![0])),
                make(Opcode::OpSetGlobal, Some(vec![0])),
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "
            fn() {
                let num = 55;
                num
            }"
            .to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(55)),
                Constant::Instructions(vec![
                    make(Opcode::OpConstant, Some(vec![0])),
                    make(Opcode::OpSetLocal, Some(vec![0])),
                    make(Opcode::OpGetLocal, Some(vec![0])),
                    make(Opcode::OpReturnValue, None),
                ]),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![1])),
                make(Opcode::OpPop, None),
            ],
        },
        TestCase {
            input: "fn() {
                    let a = 55;
        let b = 77;
        a+b }"
                .to_string(),
            expected_constants: vec![
                Constant::Object(Object::Integer(55)),
                Constant::Object(Object::Integer(77)),
                Constant::Instructions(vec![
                    make(Opcode::OpConstant, Some(vec![0])),
                    make(Opcode::OpSetLocal, Some(vec![0])),
                    make(Opcode::OpConstant, Some(vec![1])),
                    make(Opcode::OpSetLocal, Some(vec![1])),
                    make(Opcode::OpGetLocal, Some(vec![0])),
                    make(Opcode::OpGetLocal, Some(vec![1])),
                    make(Opcode::OpAdd, None),
                    make(Opcode::OpReturnValue, None),
                ]),
            ],
            expected_instructions: vec![
                make(Opcode::OpConstant, Some(vec![2])),
                make(Opcode::OpPop, None),
            ],
        },
    ];

    run_tests(tests);
}

fn test_string_object(expected: String, actual: Object) {
    match actual {
        Object::String(v) => assert_eq!(expected, v),
        _ => panic!("object is not String. got={:?}", actual),
    }
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

fn test_constants(expected: Vec<Constant>, actual: Vec<Object>) {
    for (i, constant) in expected.iter().enumerate() {
        match constant {
            Constant::Object(obj) => match obj {
                Object::Integer(v) => test_int_object(*v, actual[i].clone()),
                Object::Boolean(_) => todo!(),
                Object::String(v) => test_string_object(v.to_string(), actual[i].clone()),
                Object::Array(_) => todo!(),
                Object::Hash(_) => todo!(),
                Object::Function(_, _, _) => todo!(),
                Object::Builtin(_, _, _) => todo!(),
                Object::Null => todo!(),
                Object::ReturnValue(_) => todo!(),
                Object::Error(_) => todo!(),
                Object::CompiledFn(_, _, _) => todo!(),
            },
            Constant::Instructions(ins) => {
                let func = actual[i].clone();
                let result = match func {
                    Object::CompiledFn(actual_ins, _, _) => actual_ins,
                    _ => unimplemented!(),
                };
                test_instructions(ins.to_vec(), result);
            }
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
