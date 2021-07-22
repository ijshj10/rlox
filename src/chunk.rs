use std::convert::TryInto;

pub use crate::value::Value;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_return() {
        let mut chunk = Chunk::new();
        chunk.insert(OpCode::Return, 0);

        assert_eq!(OpCode::from_bytes(&chunk.bytes), OpCode::Return);
    }
    #[test]
    fn test_add_constant() {
        let mut chunk = Chunk::new();
        let value = 1.2;
        let index = chunk.add_constant(value);
        chunk.insert(OpCode::Constant(index), 0);

        assert_eq!(
            OpCode::from_bytes(&chunk.bytes),
            OpCode::Constant(index as u8)
        );
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpCode {
    Return,
    Constant(u8),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Unknown,
}

impl OpCode {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        match bytes[0] {
            0 => (Self::Return),
            1 => (Self::Constant(bytes[1])),
            2 => (Self::Negate),
            3 => Self::Add,
            4 => Self::Subtract,
            5 => Self::Multiply,
            6 => Self::Divide,
            _ => (Self::Unknown),
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::Return => vec![0],
            Self::Constant(index) => vec![1, index],
            Self::Negate => vec![2],
            Self::Add => vec![3],
            Self::Subtract => vec![4],
            Self::Multiply => vec![5],
            Self::Divide => vec![6],
            Self::Unknown => unreachable!(),
        }
    }

    pub(crate) fn to_string(self, chunk: &Chunk) -> String {
        let mut s = format!("{:?}", self);

        if let Self::Constant(idx) = self {
            s.push_str(&format!(" {}", chunk.constants[idx as usize]));
        }
        s
    }

    pub(crate) fn len(self) -> usize {
        match self {
            Self::Constant(_) => 2,
            _ => 1,
        }
    }
}

pub struct Chunk {
    pub(crate) bytes: Vec<u8>,
    pub(crate) constants: Vec<Value>,
    pub(crate) lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    pub fn insert(&mut self, op_code: OpCode, line: usize) {
        self.bytes.append(&mut op_code.into_bytes());
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);

        (self.constants.len() - 1)
            .try_into()
            .expect("Too many constants")
    }

    pub fn disassmble(&self, name: &str) {
        println!("== {} ==", name);

        let mut current_offset = 0;
        let mut current_instruction = 0;

        while current_offset < self.bytes.len() {
            let op = OpCode::from_bytes(&self.bytes[current_offset..]);
            let offset = op.len();

            print!("{:04} ", current_offset);
            if current_offset > 0
                && self.lines[current_instruction - 1] == self.lines[current_instruction]
            {
                print!("   | ");
            } else {
                print!("{:04} ", self.lines[current_instruction]);
            }
            println!("{:?}", op.to_string(&self));

            current_offset += offset;
            current_instruction += 1;
        }
    }
}
