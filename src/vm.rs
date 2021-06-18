use crate::instruction::Opcode;

const REGISTERS_COUNT: usize = 32;

#[derive(Debug)]
pub struct VM {
    pc: usize,
    pub registers: [i32; REGISTERS_COUNT],
    pub program: Vec<u8>
}

impl VM {
    pub fn new() -> Self {
        VM {
            pc: 0,
            registers: [0; REGISTERS_COUNT],
            program: vec![]
        }
    }

    fn decode_instruction(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let byte = self.program[self.pc];
        self.pc += 1;
        byte
    }

    fn next_16_bits(&mut self) -> u16 {
        let first_byte = self.next_8_bits() as u16;
        let second_byte = self.next_8_bits() as u16;
        (first_byte << 8) | second_byte
    }

    pub fn run(&mut self) {
        let mut is_done = false;

        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn execute_instruction(&mut self) -> bool {
        if self.pc > self.program.len() {
            return true;
        }

        match self.decode_instruction() {
            Opcode::HLT  => {
                println!("Program ended");
                return true;
            }
            Opcode::MOVI => {
                // e.g movi reg, 0x10
                let register = self.next_8_bits() as usize;
                let value = self.next_16_bits();

                self.registers[register] = value as i32;
            }
            Opcode::MOV => {
                // e.g movi reg1, reg2
                let dest_reg = self.next_8_bits() as usize;
                let source_reg = self.next_8_bits() as usize;

                self.registers[dest_reg] = self.registers[source_reg];
            }
            Opcode::ADD => {
                // e.g add reg, 0x10
                let register = self.next_8_bits() as usize;
                let value = self.next_16_bits();

                self.registers[register] += value as i32;
            }
            Opcode::SUB => {
                // e.g sub reg, 0x10
                let register = self.next_8_bits() as usize;
                let value = self.next_16_bits();

                self.registers[register] -= value as i32;
            }
            Opcode::AND => {
                // e.g and reg1, reg2
                let dest_reg = self.next_8_bits() as usize;
                let source_reg = self.next_8_bits() as usize;

                self.registers[dest_reg] &= self.registers[source_reg];
            }
            Opcode::OR => {
                // e.g or reg1, reg2
                let dest_reg = self.next_8_bits() as usize;
                let source_reg = self.next_8_bits() as usize;

                self.registers[dest_reg] |= self.registers[source_reg];
            }
            Opcode::XOR => {
                // e.g xor reg1, reg2
                let dest_reg = self.next_8_bits() as usize;
                let source_reg = self.next_8_bits() as usize;

                self.registers[dest_reg] ^= self.registers[source_reg];
            }
            Opcode::JMP => {
                let jmp_to = self.next_8_bits() as usize;
                self.pc = jmp_to;
            }
            Opcode::INVALID => {
                println!("Unrecognized instruction");
                return true;
            }
        }

        return false;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_init() {
        let vm = VM::new();
        assert_eq!(0, vm.pc);
        assert_eq!(true, vm.program.is_empty());
        assert_eq!([0; 32], vm.registers);
    }

    #[test]
    fn test_hlt_instrucion() {
        let mut vm = VM::new();
        // Making sure that after a HLT instruction the execution doesn't continue
        vm.program = vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        vm.run();
        assert_eq!(1, vm.pc);
    }

    #[test]
    fn test_invalid_instruction() {
        let mut vm = VM::new();
        // Making sure that after an invalid instruction the execution doesn't continue
        vm.program = vec![0x90, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        vm.run();
        assert_eq!(1, vm.pc);
    }

    #[test]
    fn test_movi_instruction() {
        let mut vm = VM::new();

        // mov reg_4, 512
        vm.program = vec![0x01, 0x4, 0x02, 0x00];
        vm.execute_instruction();
        assert_eq!(vm.registers[4], 512);
    }

    #[test]
    fn test_mov_instruction() {
        let mut vm = VM::new();

        // movi reg_1, 512
        vm.program = vec![0x01, 0x1, 0x02, 0x00];
        vm.execute_instruction();

        // movi reg_2, 256
        vm.program.extend_from_slice(&[0x1, 0x2, 0x01, 0x00]);
        vm.execute_instruction();

        // mov reg_2, reg1
        vm.program.extend_from_slice(&[0x2, 0x2, 0x01]);
        vm.execute_instruction();

        assert_eq!(vm.registers[1], 512);
        assert_eq!(vm.registers[2], 512);
    }

    #[test]
    fn test_add_instruction() {
        let mut vm = VM::new();

        // add reg_1, 512
        vm.program = vec![0x3, 0x1, 0x02, 0x00];
        vm.execute_instruction();
        assert_eq!(vm.registers[1], 512);
    }

    #[test]
    fn test_sub_instruction() {
        let mut vm = VM::new();

        // add reg_1, 512
        vm.program = vec![0x3, 0x1, 0x02, 0x00];
        vm.execute_instruction();

        // sub reg_1, 256
        vm.program.extend_from_slice(&[0x4, 0x1, 0x01, 0x00]);
        vm.execute_instruction();

        assert_eq!(vm.registers[1], 256);
    }

    #[test]
    fn test_and_instruction() {
        let mut vm = VM::new();

        // add reg_1, 0x07
        vm.program = vec![0x3, 0x1, 0x00, 0x07];
        vm.execute_instruction();

        // add reg_2, 0xa
        vm.program.extend_from_slice(&[0x3, 0x2, 0x00, 0x0a]);
        vm.execute_instruction();

        // and reg_1, reg2
        vm.program.extend_from_slice(&[0x5, 0x1, 0x02]);
        vm.execute_instruction();

        assert_eq!(vm.registers[1], 0x2);
        assert_eq!(vm.registers[2], 0xa);
    }

    #[test]
    fn test_xor_instruction() {
        let mut vm = VM::new();

        // add reg_1, 0x07
        vm.program = vec![0x3, 0x1, 0x00, 0x07];
        vm.execute_instruction();

        // add reg_2, 0xa
        vm.program.extend_from_slice(&[0x3, 0x2, 0x00, 0x0a]);
        vm.execute_instruction();

        // xor reg_1, reg2
        vm.program.extend_from_slice(&[0x7, 0x1, 0x02]);
        vm.execute_instruction();

        assert_eq!(vm.registers[1], 13);
        assert_eq!(vm.registers[2], 0xa);
    }
}