pub mod instruction;

use gorp_asm::parse_instruction;
use self::instruction::Instruction;

#[derive(Debug, PartialEq)]
pub enum State {
    Suspended,
    Running,
    Halting,
}

pub struct Cpu {
    state: State,
    pc: usize,
    registers: [usize; 16],
    rom: Vec<Instruction>,
    memory: Vec<usize>,
}

impl Cpu {
    pub fn new() -> Self { Default::default() }

    pub fn run(&mut self) {
        self.state = State::Running;
        while self.pc < self.rom.len() && self.state == State::Running {
            let next_instruction = self.rom[self.pc];
            self.process_instruction(next_instruction);
            self.pc += 1;
        }
    }

    pub fn load_bytes(&mut self, bytes: &[u8]) {
        use std::io::Read;
        use std::io::BufReader;

        self.rom = Vec::with_capacity(bytes.len() / 4);

        let mut instructions = Vec::new();
        let mut reader = BufReader::new(bytes);
        let mut buffer = [0; 4];
        while let Ok(()) = reader.read_exact(&mut buffer) {
            instructions.push(Instruction::from(buffer));
        }
        self.load_instructions(instructions);
    }

    pub fn load_instructions(&mut self, instructions: Vec<Instruction>) {
        self.rom = instructions;
    }

    pub fn load_assembly(&mut self, assembly: &str) {
        let instructions: Vec<Instruction> = assembly
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(parse_instruction)
            .map(Instruction::from)
            .collect();

        self.load_instructions(instructions);
    }

    pub fn load_assembly_file<P: AsRef<std::path::Path>>(&mut self, path: P) {
        let assembly = std::fs::read_to_string(path).expect("Error reading file");
        self.load_assembly(&assembly);
    }

    pub fn registers(&self) -> &[usize] {
        &self.registers
    }

    pub fn memory(&self) -> &[usize] {
        &self.memory
    }

    fn process_instruction(&mut self, instruction: Instruction) {
        // dbg!(instruction.as_assembly());
        let (dest, op1, op2) = self.evaluate_all_parameters(instruction);

        match instruction.opcode {
            0x00 => self.state = State::Halting,
            0x01 => self.registers[dest] = self.memory[op1],
            0x02 => self.memory[dest] = self.registers[op1],
            0x03 => self.registers[dest] = op1,
            0x04 => self.registers[dest] = self.registers[op1],
            0x10 => if op1 > 0 {
                if op2 == 0 {
                    self.pc -= dest
                } else {
                    self.pc += dest
                }
            },
            0x11 => if op1 == 0 {
                if op2 == 0 {
                    self.pc -= dest
                } else {
                    self.pc += dest
                }
            },
            0x20 => self.registers[dest] = op1 + op2,
            0x21 => self.registers[dest] = op1 - op2,
            0x22 => self.registers[dest] = op1 * op2,
            0x23 => self.registers[dest] = op1 / op2,
            0x24 => self.registers[dest] = op1 % op2,
            0x30 => self.registers[dest] = if op1 == op2 { 1 } else { 0 },
            0x31 => self.registers[dest] = if op1 != op2 { 1 } else { 0 },
            0x32 => self.registers[dest] = if op1 < op2 { 1 } else { 0 },
            0x33 => self.registers[dest] = if op1 <= op2 { 1 } else { 0 },
            0x34 => self.registers[dest] = if op1 > op2 { 1 } else { 0 },
            0x35 => self.registers[dest] = if op1 >= op2 { 1 } else { 0 },
            0x50 => {
                use std::io::{self, Read};

                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer).expect("Error reading stdin");
                let value = buffer.parse::<usize>().expect("Input could not be parsed to a usize");
                self.registers[dest] = value;
            },
            0x51 => {
                use std::io::{self, Write};

                let output = self.registers[dest].to_string();
                io::stdout().write_all(output.as_bytes()).expect("Error writing to stdout");
            },
            _ => panic!("Unknown instruction: {}", instruction.opcode),
        }
        // dbg!(&self);
    }

    fn evaluate_parameter(&self, parameter: u8) -> usize {
        let mode = (parameter & 0xF0) >> 4;
        if mode >= 0b1000 {
            // Immediate mode
            (parameter & 0b0111_1111) as usize
        } else if mode >= 0b0100 {
            // Offset mode
            // Not sure how I'm going to use this yet or what it's even for
            // I read a bit about addressing but I want to implement some stuff
            // before I read further
            let offset = (parameter & 0b0011_1111) as usize;
            self.pc + offset
        } else if mode >= 0b0001 {
            // Register mode
            self.registers[(parameter & 0x0F) as usize]
        } else if mode == 0 {
            // Immediate mode
            (parameter & 0b0000_1111) as usize
        } else {
            panic!("Invalid mode: {}", mode);
        } 
    }

    fn evaluate_all_parameters(&self, instruction: Instruction) -> (usize, usize, usize) {
        let (_, dest, op1, op2) = instruction.into_parts();
        (self.evaluate_parameter(dest), self.evaluate_parameter(op1), self.evaluate_parameter(op2))
    }

    // pub fn input(&mut self) {
    //     use std::io::Read;
    //     let mut buffer = String::new();
    //     std::io::stdin().read_to_string(&mut buffer).expect("Error reading stdin");
    // }

    // pub fn output<'a, T: AsRef<[u8]>>(&self, value: T) {
    //     use std::io::Write;
    //     std::io::stdout().write_all(value.as_ref()).expect("Error write to stdout");
    // }
}

impl Default for Cpu {
    fn default() -> Self {
        Self { 
            state: State::Suspended,
            pc: 0,
            registers: [0; 16],
            memory: vec![0; 65536],
            rom: Vec::new(),
        }
    }
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Cpu {{")?;
        writeln!(f, "\tstate: {:?}", self.state)?;
        writeln!(f, "\tpc: {}", self.pc)?;
        writeln!(f, "\tregisters: {:?}", &self.registers)?;
        writeln!(f, "\tmemory: {} values", self.memory.len())?;
        writeln!(f, "\trom: {} instructions", self.rom.len())?;
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl std::ops::ShlAssign<Instruction> for Cpu {
    fn shl_assign(&mut self, rhs: Instruction) {
        self.process_instruction(rhs);
    }
}

impl std::ops::ShlAssign<[u8; 4]> for Cpu {
    fn shl_assign(&mut self, rhs: [u8; 4]) {
        self.process_instruction(Instruction::from(rhs));
    }
}

impl std::ops::ShlAssign<&str> for Cpu {
    fn shl_assign(&mut self, rhs: &str) {
        self.process_instruction(Instruction::from(rhs))
    }
}

/* 
 *
 *  Tests
 *
 */

#[cfg(test)]
mod tests {
    use super::*;
    
    // Add register mode to a number
    fn r(num: u8) -> u8 {
        0b0001_0000 | num
    }
  
    #[test]
    fn load() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 1;
        cpu.memory[1] = 2;

        cpu <<= [0x01, 0, 0, 0];
        cpu <<= [0x01, 1, 1, 0];

        assert_eq!(cpu.registers[0], 1);
        assert_eq!(cpu.registers[1], 2);
    }

    #[test]
    fn store() {
        let mut cpu = Cpu::new();
        cpu.registers[0] = 1;
        cpu.registers[1] = 2;

        cpu <<= [0x02, 0, 0, 0];
        cpu <<= [0x02, 1, 1, 0];

        assert_eq!(cpu.memory[0], 1);
        assert_eq!(cpu.memory[1], 2);
    }

    #[test]
    fn set() {
        let mut cpu = Cpu::new();
        cpu <<= [0x03, 0, 3, 0];
        cpu <<= [0x03, 1, 8, 0];

        assert_eq!(cpu.registers[0], 3);
        assert_eq!(cpu.registers[1], 8);
    }

    #[test]
    fn copy() {
        let mut cpu = Cpu::new();
        cpu <<= [0x03, 0, 3, 0];
        cpu <<= [0x04, 1, 0, 0];
        cpu <<= [0x04, 2, 1, 0];

        assert_eq!(cpu.registers[1], 3);
        assert_eq!(cpu.registers[2], 3);
    }

    #[test]
    fn add() {
        let mut cpu = Cpu::new();
        cpu <<= [0x03, 0, 3, 0];
        cpu <<= [0x03, 1, 2, 0];

        cpu <<= [0x20, 2, r(0), r(1)];
        assert_eq!(cpu.registers[2], 5);

        cpu <<= [0x20, 2, r(0), r(2)];
        assert_eq!(cpu.registers[2], 8);

        cpu <<= [0x20, 2, r(2), 2];
        assert_eq!(cpu.registers[2], 10);
    }

    #[test]
    fn subtract() {
        let mut cpu = Cpu::new();
        cpu <<= [0x03, 0, 3, 0];
        cpu <<= [0x03, 1 ,2, 0];

        cpu <<= [0x21, 2, r(0), r(1)];
        assert_eq!(cpu.registers[2], 1);

        cpu <<= [0x03, 1, 1, 0];
        cpu <<= [0x21, 2, r(2), r(1)];
        assert_eq!(cpu.registers[2], 0);

        cpu <<= [0x21, 3, r(0), 1];
        assert_eq!(cpu.registers[3], 2);
    }
    
    #[test]
    fn multiply() {
        let mut cpu = Cpu::new();
        cpu <<= [0x03, 0, 3, 0];
        cpu <<= [0x03, 1, 2, 0];

        cpu <<= [0x22, 2, r(0), r(1)];
        assert_eq!(cpu.registers[2], 6);

        cpu <<= [0x22, 2, r(0), r(2)];
        assert_eq!(cpu.registers[2], 18);

        cpu <<= [0x22, 3, r(0), 2];
        assert_eq!(cpu.registers[3], 6);
    }

    #[test]
    fn divide() {
        let mut cpu = Cpu::new();
        cpu <<= [0x03, 0, 6, 0];
        cpu <<= [0x03, 1, 2, 0];

        cpu <<= [0x23, 2, r(0), r(1)];
        assert_eq!(cpu.registers[2], 3);

        cpu <<= [0x23, 2, r(2), r(1)];
        assert_eq!(cpu.registers[2], 1);
        
        cpu <<= [0x23, 3, r(0), 3];
        assert_eq!(cpu.registers[3], 2);
    }

    #[test]
    fn modulo() {
        let mut cpu = Cpu::new();
        cpu <<= [0x03, 0, 8, 0];
        cpu <<= [0x03, 1, 3, 0];

        cpu <<= [0x24, 2, r(0), r(1)];
        assert_eq!(cpu.registers[2], 2);

        cpu <<= [0x24, 2, r(1), r(2)];
        assert_eq!(cpu.registers[2], 1);

        cpu <<= [0x24, 3, r(0), 5];
        assert_eq!(cpu.registers[3], 3);
    }

    #[test]
    fn jump_if_true() {
        let mut cpu = Cpu::new();
        cpu.registers[0] = 1;
        cpu.registers[1] = 0;

        cpu <<= [0x10, 5, r(0), 1];
        assert_eq!(cpu.pc, 5);

        cpu <<= [0x10, 5, r(1), 1];
        assert_eq!(cpu.pc, 5);

        cpu <<= [0x10, 3, r(0), 0];
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn jump_if_false() {
        let mut cpu = Cpu::new();
        cpu.registers[0] = 0;
        cpu.registers[1] = 1;

        cpu <<= [0x11, 5, r(0), 1];
        assert_eq!(cpu.pc, 5);

        cpu <<= [0x11, 5, r(1), 1];
        assert_eq!(cpu.pc, 5);

        cpu <<= [0x11, 3, r(0), 0];
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn is_equal() {
        let mut cpu = Cpu::new();
        cpu.registers[0] = 5;
        cpu.registers[2] = 5;

        cpu <<= [0x30, 1, r(0), 5];
        assert_eq!(cpu.registers[1], 1);

        cpu <<= [0x30, 2, r(0), 4];
        assert_eq!(cpu.registers[2], 0);
    }

    #[test]
    fn is_not_equal() {
        let mut cpu = Cpu::new();
        cpu.registers[0] = 5;
        cpu.registers[2] = 5;

        cpu <<= [0x31, 1, r(0), 4];
        assert_eq!(cpu.registers[1], 1);

        cpu <<= [0x31, 2, r(0), 5];
        assert_eq!(cpu.registers[2], 0);
    }

    #[test]
    fn is_less_than() {
        let mut cpu = Cpu::new();
        cpu.registers[0] = 5;
        cpu.registers[2] = 5;

        cpu <<= [0x32, 1, r(0), 6];
        assert_eq!(cpu.registers[1], 1);

        cpu <<= [0x32, 2, r(0), 4];
        assert_eq!(cpu.registers[2], 0);
    }

    #[test]
    fn is_less_than_or_equal() {
        let mut cpu = Cpu::new();
        cpu.registers[0] = 5;
        cpu.registers[2] = 5;

        cpu <<= [0x33, 1, r(0), 6];
        assert_eq!(cpu.registers[1], 1);

        cpu <<= [0x33, 1, r(0), 5];
        assert_eq!(cpu.registers[1], 1);

        cpu <<= [0x33, 2, r(0), 4];
        assert_eq!(cpu.registers[2], 0);
    }

    #[test]
    fn is_greater_than() {
        let mut cpu = Cpu::new();
        cpu.registers[0] = 5;
        cpu.registers[2] = 5;

        cpu <<= [0x34, 1, r(0), 3];
        assert_eq!(cpu.registers[1], 1);

        cpu <<= [0x34, 2, r(0), 5];
        assert_eq!(cpu.registers[2], 0);

        cpu <<= [0x34, 2, r(0), 7];
        assert_eq!(cpu.registers[2], 0);
    }

    #[test]
    fn is_greater_than_or_equal() {
        let mut cpu = Cpu::new();
        cpu.registers[0] = 5;
        cpu.registers[2] = 5;

        cpu <<= [0x35, 1, r(0), 5];
        assert_eq!(cpu.registers[1], 1);

        cpu <<= [0x35, 1, r(0), 4];
        assert_eq!(cpu.registers[1], 1);

        cpu <<= [0x35, 2, r(0), 7];
        assert_eq!(cpu.registers[2], 0);
    }

    #[test]
    fn looping_addition_program() {
        let mut cpu = Cpu::new();
        cpu.load_bytes(&[
            0x03, 0, 1, 0,
            0x03, 1, 1, 0,
            0x20, 1, r(0), r(1),
            0x32, 3, r(1), 8,
            0x10, 3, r(3), 0,
            0x11, 2, r(2), 1,
            0, 0, 0, 0,
            0, 0, 0, 0,
            0x03, 2, 9, 0,
            0,
        ]);
        cpu.run();

        assert_eq!(cpu.registers[0], 1);
        assert_eq!(cpu.registers[1], 8);
        assert_eq!(cpu.registers[2], 9);
        assert_eq!(cpu.registers[3], 0);
    }
}
