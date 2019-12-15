#[derive(Debug)]
pub enum Instruction {
    Halt,
    Load(usize, usize),
    Store(usize, usize),
    JumpIfTrue(usize),
    JumpIfFalse(usize),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equals,
    LessThan,
}

impl From<&[usize]> for Instruction {
    fn from(input: &[usize]) -> Self {
        use Instruction::*;

        let opcode = input[0];

        match opcode {
            0 => Halt,
            1 => Load(input[1], input[2]),
            2 => Store(input[1], input[2]),
            3 => JumpIfTrue(input[1]),
            4 => JumpIfFalse(input[1]),
            5 => Add,
            6 => Sub,
            7 => Mul,
            9 => Div,
            10 => Mod,
            11 => Equals,
            12 => LessThan,
            _ => panic!("Invalid opcode: {}", opcode),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
