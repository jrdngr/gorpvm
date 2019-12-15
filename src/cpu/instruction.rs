pub enum Instruction {
    Load(usize, usize),
    Store(usize, usize),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Halt,
}

impl From<&[i64]> for Instruction {
    fn from(input: &[i64]) -> Self {
        use Instruction::*;

        let opcode = input[0];

        match opcode {
            0 => Halt,
            1 => Load(input[1] as usize, input[2] as usize),
            2 => Store(input[1] as usize, input[2] as usize),
            3 => Add,
            4 => Sub,
            5 => Mul,
            6 => Div,
            7 => Mod,
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
