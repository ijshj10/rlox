use lox::*;

fn main() {
    let mut chunk = Chunk::new();
    //let constant = chunk.add_constant(2 as Value);
    let constant = chunk.add_constant(3 as Value);
    chunk.write(OpCode::ConstantLong as u8, 0);
    chunk.write((constant >> 16) as u8, 0);
    chunk.write((constant >> 8) as u8, 0);
    chunk.write(constant as u8, 0);
    chunk.write(OpCode::Return as u8, 0);

    chunk.disassemble("test")
}
