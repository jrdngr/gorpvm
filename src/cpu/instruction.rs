#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Instruction {
    pub opcode: u8,
    pub src1: u8,
    pub src2: u8,
    pub dest: u8,
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

impl Instruction {
    pub fn into_parts(self) -> (u8, u8, u8, u8) {
        (self.opcode, self.src1, self.src2, self.dest)
    }
}


/*

[  00000000  |  0000_0000  |  0000_0000  |  0000_0000  ]
[  opcode    |  mode_src1  |  mode_src2  |  mode_dest  ]  

0000 - unused
0001 - register
001x - ?
01xx - offset
1xxx - immediate


00 - halt

01 - load reg mem
02 - stor reg mem
03 - set reg val

10 - jmpt test reg
11 - jmpf test reg

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
}
