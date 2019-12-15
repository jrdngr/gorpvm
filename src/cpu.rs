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
    pc: usize,
    state: State,
    pub registers: [u8; 16],
    pub memory: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Self { Default::default() }

    pub fn run(&mut self) {
        self.state = State::Running;
        while self.state == State::Running {
            let next_instruction = Instruction::from(&self.memory[self.pc..self.pc+3]);
            self.evaluate(next_instruction);
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.set_memory_slice(0, program);
    }

    pub fn evaluate(&mut self, instruction: Instruction) {
        use Instruction::*;

        match instruction {
            Halt => self.state = State::Halting,
            Load(mem, reg) => self.registers[reg] = self.memory[mem],
            Store(reg, mem) => self.memory[mem] = self.registers[reg],
            JumpIfTrue(pos) => if self.registers[0] > 0 { self.pc = pos },
            JumpIfFalse(pos) => if self.registers[0] == 0 { self.pc = pos },
            Add => self.registers[0] = self.registers[0] + self.registers[1],
            Sub => self.registers[0] = self.registers[0] - self.registers[1],
            Mul => self.registers[0] = self.registers[0] * self.registers[1],
            Div => self.registers[0] = self.registers[0] / self.registers[1],
            Mod => self.registers[0] = self.registers[0] % self.registers[1],
            Equals => self.registers[0] = if self.registers[0] == self.registers[1] { 1 } else { 0 },
            LessThan => self.registers[0] = if self.registers[0] < self.registers[1]{ 1 } else { 0 },
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

    pub fn set_registers(&mut self, values: &[u8]) {
        let end = values.len().min(self.registers.len());
        for i in 0..end {
            self.registers[i] = values[i];
        }
    }

    pub fn set_memory_slice(&mut self, start: usize, values: &[u8]) {
        clone_slice_into_index(values, &mut self.memory, start);
    }
}

impl Default for  Cpu {
    fn default() -> Self {
        Self { 
            pc: 0,
            state: State::Suspended,
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
    use super::*;
    
    #[test]
    fn load() {
        use Instruction::Load;

        let mut cpu = Cpu::new();
        cpu.set_memory_slice(0, &[1, 2, 3]);

        cpu.evaluate(Load(0, 0));
        cpu.evaluate(Load(1, 1));
        cpu.evaluate(Load(2, 2));

        assert_eq!(cpu.registers[0], 1);
        assert_eq!(cpu.registers[1], 2);
        assert_eq!(cpu.registers[2], 3);

        cpu.evaluate(Load(2, 0));
        assert_eq!(cpu.registers[0], 3);
    }

    #[test]
    fn store() {
        use Instruction::Store;

        let mut cpu = Cpu::new();
        cpu.set_registers(&[1, 2, 3]);

        cpu.evaluate(Store(0, 0));
        cpu.evaluate(Store(1, 1));
        cpu.evaluate(Store(2, 2));

        assert_eq!(cpu.memory[0], 1);
        assert_eq!(cpu.memory[1], 2);
        assert_eq!(cpu.memory[2], 3);

        cpu.evaluate(Store(2, 0));
        assert_eq!(cpu.memory[0], 3);
    }

    #[test]
    fn add() {
        use Instruction::Add;

        let mut cpu = Cpu::new();
        cpu.set_registers(&[1, 2]);
        cpu.evaluate(Add);

        assert_eq!(cpu.registers[0], 3);
    }

    #[test]
    fn subtract() {
        use Instruction::Sub;

        let mut cpu = Cpu::new();
        cpu.set_registers(&[5, 2]);
        cpu.evaluate(Sub);

        assert_eq!(cpu.registers[0], 3);
    }

    #[test]
    fn multiply() {
        use Instruction::Mul;

        let mut cpu = Cpu::new();
        cpu.set_registers(&[3, 5]);
        cpu.evaluate(Mul);

        assert_eq!(cpu.registers[0], 15);
    }

    #[test]
    fn divide() {
        use Instruction::Div;

        let mut cpu = Cpu::new();
        cpu.set_registers(&[8, 2]);
        cpu.evaluate(Div);

        assert_eq!(cpu.registers[0], 4);
    }

    #[test]
    fn modulo() {
        use Instruction::Mod;

        let mut cpu = Cpu::new();
        cpu.set_registers(&[7, 3]);
        cpu.evaluate(Mod);

        assert_eq!(cpu.registers[0], 1);
    }
}
