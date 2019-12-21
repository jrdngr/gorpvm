#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Instruction {
    pub opcode: u8,
    pub src1: u8,
    pub src2: u8,
    pub dest: u8,
}

impl From<[u8; 4]> for Instruction {
    fn from(bytes: [u8; 4]) -> Self {
        Self {
            opcode: bytes[0],
            src1: bytes[1],
            src2: bytes[2],
            dest: bytes[3],
        }
    }
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        Self {
            opcode: ((value & 0xFF000000) >> 24) as u8,
            src1:   ((value & 0x00FF0000) >> 16) as u8,
            src2:   ((value & 0x0000FF00) >> 8) as u8,
            dest:    (value & 0x000000FF) as u8,
        }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        crate::cpu::assembly::parse_instruction(value)
    }
}

impl From<Instruction> for u32 {
    fn from(instruction: Instruction) -> u32 {
        (instruction.opcode as u32) << 24 |
        (instruction.src1 as u32)   << 16 |
        (instruction.src2 as u32)   << 8  |
        (instruction.dest as u32)
    }
}

impl Instruction {
    pub fn into_parts(self) -> (u8, u8, u8, u8) {
        (self.opcode, self.src1, self.src2, self.dest)
    }

    pub fn as_assembly(&self) -> String {
        let instruction = match self.opcode {
            0x00 => "hlt",
            0x01 => "ldr",
            0x02 => "str",
            0x03 => "set",
            0x04 => "cpy",
            0x10 => "jpt",
            0x11 => "jpf",
            0x20 => "add",
            0x21 => "sub",
            0x22 => "mul",
            0x23 => "div",
            0x24 => "mod",
            0x30 => "eql",
            0x31 => "neq",
            0x32 => "let",
            0x33 => "leq",
            0x34 => "grt",
            0x35 => "geq",
            _    => "invalid",
        };

        let src1 = Self::parameter_as_str(self.src1);
        let src2 = Self::parameter_as_str(self.src2);
        let dest = Self::parameter_as_str(self.dest);

        format!("{} {} {} {}", instruction, src1, src2, dest)
    }

    fn parameter_as_str(parameter: u8) -> String {
        let mode = (parameter & 0xF0) >> 4;
        if mode >= 0b1000 {
            format!("{}i", parameter & 0b0111_1111)
        } else if mode >= 0b0100 {
            format!("{}o", parameter & 0b0011_1111)
        } else if mode >= 0b0001 {
            format!("{}r", parameter & 0x0F)
        } else if mode == 0 {
            format!("{}i", parameter & 0x0F)
        } else {
            String::from("xxx")
        } 
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_assembly())
    }
}

/*

[  00000000  |  0000_0000  |  0000_0000  |  0000_0000  ]
[  opcode    |  mode_src1  |  mode_src2  |  mode_dest  ]  

0000 - immediate
0001 - register
001x - ?
01xx - offset
1xxx - immediate


00 - halt

01 - load reg mem
02 - stor reg mem
03 - set reg val
04 - copy reg1 reg2

10 - jmpt test sign pos
11 - jmpf test sign pos

20 - add x y dest
21 - sub x y dest
22 - mul x y dest
23 - div x y dest
24 - mod x y dest

30 - eq x y dest
31 - ne x y dest
32 - lt x y dest
33 - le x y dest
34 - gt x y dest
35 - ge x y dest

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u32() {
        assert_eq!(Instruction::from(0xAA112233).into_parts(), (0xAA, 0x11, 0x22, 0x33));
    }

    #[test]
    fn from_bytes() {
        assert_eq!(Instruction::from([0xAA, 0x11, 0x22, 0x33]).into_parts(), (0xAA, 0x11, 0x22, 0x33));
    }
}
