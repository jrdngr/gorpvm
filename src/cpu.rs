pub mod instruction;

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

    pub fn load_program(&mut self, program: &[u8]) {
        use std::io::Read;
        use std::io::BufReader;

        self.rom = Vec::with_capacity(program.len() / 4);

        let mut reader = BufReader::new(program);
        let mut buffer = [0; 4];
        while let Ok(()) = reader.read_exact(&mut buffer) {
            self.rom.push(Instruction::from(buffer));
        }
    }

    pub fn registers(&self) -> &[usize] {
        &self.registers
    }

    pub fn memory(&self) -> &[usize] {
        &self.memory
    }

    fn process_instruction(&mut self, instruction: Instruction) {
        let (src1, src2, dest) = self.evaluate_all_parameters(instruction);
        
        match instruction.opcode {
            0x00 => self.state = State::Halting,
            0x01 => self.registers[dest] = self.memory[src1],
            0x02 => self.memory[dest] = self.registers[src1],
            0x03 => self.registers[dest] = src1,
            0x04 => self.registers[dest] = self.registers[src1],
            0x10 => if self.registers[src1] > 0 {
                if src2 == 0 {
                    self.pc -= dest
                } else {
                    self.pc += dest
                }
            },
            0x11 => if self.registers[src1] == 0 {
                if src2 == 0 {
                    self.pc -= dest
                } else {
                    self.pc += dest
                }
            },
            0x20 => self.registers[dest] = self.registers[src1] + self.registers[src2],
            0x21 => self.registers[dest] = self.registers[src1] - self.registers[src2],
            0x22 => self.registers[dest] = self.registers[src1] * self.registers[src2],
            0x23 => self.registers[dest] = self.registers[src1] / self.registers[src2],
            0x24 => self.registers[dest] = self.registers[src1] % self.registers[src2],
            0x30 => self.registers[dest] = if self.registers[src1] == self.registers[src2] { 1 } else { 0 },
            0x31 => self.registers[dest] = if self.registers[src1] < self.registers[src2] { 1 } else { 0 },
            0x32 => self.registers[dest] = if self.registers[src1] <= self.registers[src2] { 1 } else { 0 },
            0x33 => self.registers[dest] = if self.registers[src1] > self.registers[src2] { 1 } else { 0 },
            0x34 => self.registers[dest] = if self.registers[src1] >= self.registers[src2] { 1 } else { 0 },
            _ => panic!("Unknown instruction: {}", instruction.opcode),
        }
    }

    fn evaluate_parameter(&self, parameter: u8) -> usize {
        let mode = (parameter & 0xF0) >> 4;
        if mode >= 0b1000 {
            // Immediate mode
            (parameter & 0b0111_1111) as usize
        } else if mode >= 0b0100 {
            // Offset mode
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
        let (_, src1, src2, dest) = instruction.into_parts();
        (self.evaluate_parameter(src1), self.evaluate_parameter(src2), self.evaluate_parameter(dest))
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

/* 
 *
 *  Tests
 *
 */

#[cfg(test)]
mod tests {
    use super::*;
  
    #[test]
    fn set() {
        let mut cpu = Cpu::new();
        cpu.process_instruction(Instruction::from([0x03, 3, 0, 0]));
        cpu.process_instruction(Instruction::from([0x03, 8, 0, 1]));

        assert_eq!(cpu.registers[0], 3);
        assert_eq!(cpu.registers[1], 8);
    }

    #[test]
    fn load() {
        let mut cpu = Cpu::new();
        
    }

    // #[test]
    // fn store() {
    //     use Instruction::Store;

    //     let mut cpu = Cpu::new();
    //     cpu.set_registers(&[1, 2, 3]);

    //     cpu.evaluate(Store(0, 0));
    //     cpu.evaluate(Store(1, 1));
    //     cpu.evaluate(Store(2, 2));

    //     assert_eq!(cpu.memory[0], 1);
    //     assert_eq!(cpu.memory[1], 2);
    //     assert_eq!(cpu.memory[2], 3);

    //     cpu.evaluate(Store(2, 0));
    //     assert_eq!(cpu.memory[0], 3);
    // }

    // #[test]
    // fn add() {
    //     use Instruction::Add;

    //     let mut cpu = Cpu::new();
    //     cpu.set_registers(&[1, 2]);
    //     cpu.evaluate(Add);

    //     assert_eq!(cpu.registers[0], 3);
    // }

    // #[test]
    // fn subtract() {
    //     use Instruction::Sub;

    //     let mut cpu = Cpu::new();
    //     cpu.set_registers(&[5, 2]);
    //     cpu.evaluate(Sub);

    //     assert_eq!(cpu.registers[0], 3);
    // }

    // #[test]
    // fn multiply() {
    //     use Instruction::Mul;

    //     let mut cpu = Cpu::new();
    //     cpu.set_registers(&[3, 5]);
    //     cpu.evaluate(Mul);

    //     assert_eq!(cpu.registers[0], 15);
    // }

    // #[test]
    // fn divide() {
    //     use Instruction::Div;

    //     let mut cpu = Cpu::new();
    //     cpu.set_registers(&[8, 2]);
    //     cpu.evaluate(Div);

    //     assert_eq!(cpu.registers[0], 4);
    // }

    // #[test]
    // fn modulo() {
    //     use Instruction::Mod;

    //     let mut cpu = Cpu::new();
    //     cpu.set_registers(&[7, 3]);
    //     cpu.evaluate(Mod);

    //     assert_eq!(cpu.registers[0], 1);
    // }

    // #[test]
    // fn looping_addition_program() {
    //     let mut cpu = Cpu::new();
    //     cpu.load_program(&[
    //         0x03, 1, 0, 0,
    //         0x03, 1, 0, 1,
    //         0x20, 0, 1, 1,
    //         0x31, r(1), 8, 3,
    //         0x10, r(3), 1, 4,
    //         0,
    //     ]);
    //     cpu.run();

    //     dbg!(&cpu.registers());

    //     assert_eq!(cpu.registers()[1], 8);
    // }

    // fn r(num: u8) -> u8 {
    //     0b0001_0000 | num
    // }

}
