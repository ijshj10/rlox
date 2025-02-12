use crate::common::{Chunk, OpCode, Value};

impl Chunk {
    pub fn disassemble(&self, name: &str) {
        println!("=== {} chunk ====", name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, pos: usize) -> usize {
        let code : OpCode= self.code[pos].into();
        print!("{:04} ", pos);
        if(pos > 0 && self.lines[pos] == self.lines[pos-1]) {
            print!("   | ");
        } else {
            print!("{:04} ", self.lines[pos]);
        }
        match code {
            OpCode::Return => Chunk::simple_instruction("OP_RETURN", pos),
            OpCode::Constant => self.constant_instruction("OP_CONSTANT", pos),
        }
    }

    fn simple_instruction(name: &str, pos: usize) -> usize {
        println!("{}", name);
        pos + 1
    }

    fn constant_instruction(&self, name: &str, pos: usize) -> usize {
        let constant = self.code[pos + 1] as usize;
        print!("{:16} {} ", name, constant);
        Chunk::print_value(self.constants[constant]);
        println!();
        pos + 2
    }

    fn print_value(value: Value) {
        print!("'{}'", value);
    }
}

