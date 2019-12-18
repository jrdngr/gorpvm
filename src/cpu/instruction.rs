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
            0x00 => "halt",
            0x01 => "load",
            0x02 => "stor",
            0x03 => "set",
            0x04 => "copy",
            0x10 => "jmpt",
            0x11 => "jmpf",
            0x20 => "add",
            0x21 => "sub",
            0x22 => "mul",
            0x23 => "div",
            0x24 => "mod",
            0x30 => "eq",
            0x31 => "lt",
            0x32 => "le",
            0x33 => "gt",
            0x34 => "ge",
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

20 - add rx ry rd
21 - sub rx ry rd
22 - mul rx ry rd
23 - div rx ry rd
24 - mod rx ry rd

30 - eq rx ry rd
31 - lt rx ry rd
32 - le rx ry rd
33 - gt rx ry rd
34 - ge rx ry rd

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
