pub mod instruction;

use self::instruction::Instruction;

struct Cpu {
    pc: usize,
    pub registers: [i64; 8],
    pub memory: Vec<i64>,
}

impl Cpu {
    pub fn new() -> Self { Default::default() }

    pub fn evaluate(&mut self, instruction: Instruction) {
        use Instruction::*;

        match instruction {
            Load(mem, reg) => self.registers[reg] = self.memory[mem],
            Store(reg, mem) => self.memory[mem] = self.registers[reg],
            Add => self.registers[2] = self.registers[0] + self.registers[1],
            Sub => self.registers[2] = self.registers[0] - self.registers[1],
            Mul => self.registers[2] = self.registers[0] * self.registers[1],
            Div => self.registers[2] = self.registers[0] / self.registers[1],
            Mod => self.registers[2] = self.registers[0] % self.registers[1],
        }
    }

    pub fn set_registers(&mut self, values: &[i64]) {
        let end = values.len().min(self.registers.len());
        for i in 0..end {
            self.registers[i] = values[i];
        }
    }

    pub fn set_memory_slice(&mut self, start: usize, values: &[i64]) {
        for i in 0..values.len() {
            self.memory[start + i] = values[i];
        }
    }
}

impl Default for  Cpu {
    fn default() -> Self {
        Self { 
            pc: 0,
            registers: [0; 8],
            memory: vec![0; 2014],
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

        assert_eq!(cpu.registers[2], 3);
    }

    #[test]
    fn subtract() {
        use Instruction::Sub;

        let mut cpu = Cpu::new();
        cpu.set_registers(&[1, 2]);
        cpu.evaluate(Sub);

        assert_eq!(cpu.registers[2], -1);
    }

    #[test]
    fn multiply() {
        use Instruction::Mul;

        let mut cpu = Cpu::new();
        cpu.set_registers(&[3, 5]);
        cpu.evaluate(Mul);

        assert_eq!(cpu.registers[2], 15);
    }

    #[test]
    fn divide() {
        use Instruction::Div;

        let mut cpu = Cpu::new();
        cpu.set_registers(&[8, 2]);
        cpu.evaluate(Div);

        assert_eq!(cpu.registers[2], 4);
    }

    #[test]
    fn modulo() {
        use Instruction::Mod;

        let mut cpu = Cpu::new();
        cpu.set_registers(&[7, 3]);
        cpu.evaluate(Mod);

        assert_eq!(cpu.registers[2], 1);
    }
}