pub mod instruction;

use crate::utils::clone_slice_into_index;
use self::instruction::Instruction;

#[derive(Debug, PartialEq)]
pub enum State {
    Suspended,
    Running,
    Halting,
}

#[derive(Debug)]
pub struct Cpu {
    state: State,
    pc: usize,
    ram_ptr: usize,
    pub registers: [usize; 16],
    pub memory: Vec<usize>,
}

impl Cpu {
    pub fn new() -> Self { Default::default() }

    pub fn run(&mut self) {
        self.state = State::Running;
    }

    pub fn load_program(&mut self, program: &[usize]) {
        self.set_memory_slice(0, program);
        self.ram_ptr = program.len();
    }

    pub fn evaluate(&mut self, instruction: Instruction) {
        let Instruction { opcode, src1, src2, dest } = instruction;
        match opcode {
            0x00 => self.state = State::Halting,
            0x01 => {},
            0x02 => {},
            0x10 => {},
            0x11 => {},
            0x20 => {},
            0x21 => {},
            0x22 => {},
            0x23 => {},
            0x24 => {},
            0x30 => {},
            0x31 => {},
            0x32 => {},
            _ => panic!("Unknown instruction: {}", opcode),
        }
    }

    pub fn input(&mut self, ) {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).expect("Error reading stdin");
    }

    pub fn output<'a, T: AsRef<[u8]>>(&self, value: T) {
        use std::io::Write;
        std::io::stdout().write_all(value.as_ref()).expect("Error write to stdout");
    }

    pub fn set_registers(&mut self, values: &[usize]) {
        let end = values.len().min(self.registers.len());
        for i in 0..end {
            self.registers[i] = values[i];
        }
    }

    pub fn set_memory_slice(&mut self, start: usize, values: &[usize]) {
        clone_slice_into_index(values, &mut self.memory, start);
    }
}

impl Default for  Cpu {
    fn default() -> Self {
        Self { 
            state: State::Suspended,
            pc: 0,
            ram_ptr: 0,
            registers: [0; 16],
            memory: vec![0; 2048],
        }
    }
}



/* 
 *
 *  Tests
 *
 */

#[cfg(test)]
mod tests {
//     use super::*;
    
//     #[test]
//     fn load() {
//         use Instruction::Load;

//         let mut cpu = Cpu::new();
//         cpu.set_memory_slice(0, &[1, 2, 3]);

//         cpu.evaluate(Load(0, 0));
//         cpu.evaluate(Load(1, 1));
//         cpu.evaluate(Load(2, 2));

//         assert_eq!(cpu.registers[0], 1);
//         assert_eq!(cpu.registers[1], 2);
//         assert_eq!(cpu.registers[2], 3);

//         cpu.evaluate(Load(2, 0));
//         assert_eq!(cpu.registers[0], 3);
//     }

//     #[test]
//     fn store() {
//         use Instruction::Store;

//         let mut cpu = Cpu::new();
//         cpu.set_registers(&[1, 2, 3]);

//         cpu.evaluate(Store(0, 0));
//         cpu.evaluate(Store(1, 1));
//         cpu.evaluate(Store(2, 2));

//         assert_eq!(cpu.memory[0], 1);
//         assert_eq!(cpu.memory[1], 2);
//         assert_eq!(cpu.memory[2], 3);

//         cpu.evaluate(Store(2, 0));
//         assert_eq!(cpu.memory[0], 3);
//     }

//     #[test]
//     fn add() {
//         use Instruction::Add;

//         let mut cpu = Cpu::new();
//         cpu.set_registers(&[1, 2]);
//         cpu.evaluate(Add);

//         assert_eq!(cpu.registers[0], 3);
//     }

//     #[test]
//     fn subtract() {
//         use Instruction::Sub;

//         let mut cpu = Cpu::new();
//         cpu.set_registers(&[5, 2]);
//         cpu.evaluate(Sub);

//         assert_eq!(cpu.registers[0], 3);
//     }

//     #[test]
//     fn multiply() {
//         use Instruction::Mul;

//         let mut cpu = Cpu::new();
//         cpu.set_registers(&[3, 5]);
//         cpu.evaluate(Mul);

//         assert_eq!(cpu.registers[0], 15);
//     }

//     #[test]
//     fn divide() {
//         use Instruction::Div;

//         let mut cpu = Cpu::new();
//         cpu.set_registers(&[8, 2]);
//         cpu.evaluate(Div);

//         assert_eq!(cpu.registers[0], 4);
//     }

//     #[test]
//     fn modulo() {
//         use Instruction::Mod;

//         let mut cpu = Cpu::new();
//         cpu.set_registers(&[7, 3]);
//         cpu.evaluate(Mod);

//         assert_eq!(cpu.registers[0], 1);
//     }
}
