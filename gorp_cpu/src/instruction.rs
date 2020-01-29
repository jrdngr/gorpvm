#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Instruction {
    pub opcode: u8,
    pub dest: u8,
    pub op1: u8,
    pub op2: u8,
}

impl From<[u8; 4]> for Instruction {
    fn from(bytes: [u8; 4]) -> Self {
        Self {
            opcode: bytes[0],
            dest: bytes[1],
            op1: bytes[2],
            op2: bytes[3],
        }
    }
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        Self {
            opcode: ((value & 0xFF000000) >> 24) as u8,
            dest:   ((value & 0x00FF0000) >> 16) as u8,
            op1:    ((value & 0x0000FF00) >> 8) as u8,
            op2:     (value & 0x000000FF) as u8,
        }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        Instruction::from(gorp_asm::parse_instruction(value))
    }
}

impl From<Instruction> for u32 {
    fn from(instruction: Instruction) -> u32 {
        (instruction.opcode as u32) << 24 |
        (instruction.dest as u32)   << 16 |
        (instruction.op1 as u32)    << 8  |
        (instruction.op2 as u32)
        
    }
}

impl From<Instruction> for (u8, u8, u8, u8) {
    fn from(instruction: Instruction) -> (u8, u8, u8, u8) {
        (instruction.opcode, instruction.dest, instruction.op1, instruction.op2)
    }
}

impl Instruction {
    pub fn into_parts(self) -> (u8, u8, u8, u8) {
        <(u8, u8, u8, u8)>::from(self)
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

        let dest = Self::parameter_as_str(self.dest);
        let op1 = Self::parameter_as_str(self.op1);
        let op2 = Self::parameter_as_str(self.op2);
        

        format!("{} {} {} {}", instruction, dest, op1, op2)
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
02 - stor mem reg
03 - set reg val
04 - copy destreg srcreg
10 - jmpt pos test sign 
11 - jmpf pos test sign
20 - add dest x y 
21 - sub dest x y
22 - mul dest x y
23 - div dest x y
24 - mod dest x y
30 - eq dest x y
31 - ne dest x y
32 - lt dest x y
33 - le dest x y
34 - gt dest x y
35 - ge dest x y
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