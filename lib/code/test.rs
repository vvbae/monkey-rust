use super::*;

#[test]
fn test_make() {
    let tests = vec![
        (Opcode::OpConstant, vec![65534]),
        (Opcode::OpAdd, vec![]),
        (Opcode::OpGetLocal, vec![255]),
        (Opcode::OpClosure, vec![65534, 255]),
    ];

    let expected: Vec<Vec<u8>> = vec![
        vec![Opcode::to_byte(Opcode::OpConstant), 255, 254],
        vec![Opcode::to_byte(Opcode::OpAdd)],
        vec![Opcode::to_byte(Opcode::OpGetLocal), 255],
        vec![Opcode::to_byte(Opcode::OpClosure), 255, 254, 255],
    ];

    let results = tests
        .into_iter()
        .map(|(code, operands)| make(code, Some(operands)))
        .collect::<Vec<_>>();

    assert_eq!(expected, results)
}

#[test]
fn test_instructions() {
    let instructions = vec![
        make(Opcode::OpAdd, None),
        make(Opcode::OpGetLocal, Some(vec![1])),
        make(Opcode::OpConstant, Some(vec![2])),
        make(Opcode::OpConstant, Some(vec![65535])),
        make(Opcode::OpClosure, Some(vec![65535, 255])),
    ]
    .concat();

    let expected = "0000 OpAdd
0001 OpGetLocal 1
0003 OpConstant 2
0006 OpConstant 65535
0009 OpClosure 65535 255
";

    assert_eq!(expected, string(instructions))
}

struct TestCase {
    pub op: Opcode,
    pub operands: Vec<u16>,
    pub bytes_read: u8,
}

#[test]
fn test_read_operands() {
    let tests = vec![
        TestCase {
            op: Opcode::OpConstant,
            operands: vec![65535],
            bytes_read: 2,
        },
        TestCase {
            op: Opcode::OpGetLocal,
            operands: vec![255],
            bytes_read: 1,
        },
        TestCase {
            op: Opcode::OpClosure,
            operands: vec![65535, 255],
            bytes_read: 3,
        },
    ];

    for test in tests.iter() {
        let instruction = make(test.op, Some(test.operands.clone()));

        let widths = test.op.look_up();
        let (operands_read, n) = read_operands(&widths, instruction[1..].to_vec());
        assert_eq!(test.bytes_read, n);
        assert_eq!(test.operands, operands_read);
    }
}
