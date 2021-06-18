use crate::vm::VM;

fn as_u32_be(array: &[u8]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

pub struct SimpleKeyCheck<'a> {
    vm: VM,
    key: &'a [u8; 16]
}

impl <'a> SimpleKeyCheck<'a> {
    pub fn new(key: &'a [u8; 16]) -> Self {
        SimpleKeyCheck {
            vm: VM::new(),
            key
        }
    }

    pub fn set_program(&mut self, program: &Vec<u8>) {
        self.vm.program = program.clone();
    }

    pub fn run(&mut self) -> bool {
        let iter = self.key.chunks(4);
        let mut idx: usize = 0;

        for chunk in iter {
            self.vm.registers[idx] = as_u32_be(chunk) as i32;
            idx += 1;
        }

        self.vm.run();

        // The first register is set to 1337 if the key is correct
        return 0 == self.vm.registers[0];
    }
}