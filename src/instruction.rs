#[derive(PartialEq, Debug)]
pub enum Opcode {
    HLT,
    MOVI,
    MOV,
    ADD,
    SUB,
    AND,
    OR,
    XOR,
    JMP,
    INVALID
}

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        match opcode {
            0 => Opcode::HLT,
            1 => Opcode::MOVI,
            2 => Opcode::MOV,
            3 => Opcode::ADD,
            4 => Opcode::SUB,
            5 => Opcode::AND,
            6 => Opcode::OR,
            7 => Opcode::XOR,
            8 => Opcode::JMP,
            _ => Opcode::INVALID
        }
    }
}
