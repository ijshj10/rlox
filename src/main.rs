use lox::common::*;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(2 as Value);
    chunk.write(OpCode::Constant as u8, 0);
    chunk.write(constant as u8, 0);
    chunk.write(OpCode::Return as u8, 0);

    chunk.disassemble("test")
}
