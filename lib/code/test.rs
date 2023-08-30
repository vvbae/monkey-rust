use super::*;

#[test]
fn test_make() {
    let tests = vec![(Opcode::OpConstant, vec![65534])];

    let expected: Vec<Vec<u8>> = vec![vec![Opcode::to_byte(Opcode::OpConstant), 255, 254]];

    let results = tests
        .into_iter()
        .map(|(code, operands)| make(code, operands))
        .collect::<Vec<_>>();

    assert_eq!(expected, results)
}

#[test]
fn test_instructions() {
    let instructions = vec![
        make(Opcode::OpConstant, vec![1]),
        make(Opcode::OpConstant, vec![2]),
        make(Opcode::OpConstant, vec![65535]),
    ]
    .concat();

    let expected = "0000 OpConstant 1
0003 OpConstant 2
0006 OpConstant 65535
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
    let tests = vec![TestCase {
        op: Opcode::OpConstant,
        operands: vec![65535],
        bytes_read: 2,
    }];

    for test in tests.iter() {
        let instruction = make(test.op, test.operands.clone());

        let widths = test.op.look_up().unwrap();
        let (operands_read, n) = read_operands(&widths, instruction[1..].to_vec());
        assert_eq!(n as usize, operands_read.len());

        assert_eq!(test.operands, operands_read);
    }
}
