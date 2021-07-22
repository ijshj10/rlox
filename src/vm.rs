use crate::chunk::{Chunk, OpCode, Value};

#[cfg(test)]
mod tests {
    use super::*;
}

pub struct VM<'a> {
    chunk: &'a Chunk,
    stack: Vec<Value>,
    ip: usize,
}

impl<'a> VM<'a> {
    pub fn interpret(chunk: &'a Chunk) -> InterpretResult {
        let mut vm = VM {
            chunk,
            stack: vec![],
            ip: 0,
        };

        vm.run()
    }

    fn run(&mut self) -> InterpretResult {
        while self.ip < self.chunk.bytes.len() {
            let op = OpCode::from_bytes(&self.chunk.bytes[self.ip..]);
            #[cfg(feature = "debug_trace")]
            {
                print!("          ");
                self.stack.iter().rev().for_each(|value| {
                    print!("[ {} ]", value);
                });
                println!();
                println!("{}", op.to_string(&self.chunk));
            }
            self.ip += op.len();
            match op {
                OpCode::Return => {
                    println!("{:?}", self.stack.pop());
                    return InterpretResult::Ok;
                }
                OpCode::Constant(idx) => self.stack.push(self.chunk.constants[idx as usize]),
                OpCode::Negate => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(-value);
                }
                OpCode::Add => self.binary_op(|a, b| a + b),
                OpCode::Subtract => self.binary_op(|a, b| a - b),
                OpCode::Multiply => self.binary_op(|a, b| a * b),
                OpCode::Divide => self.binary_op(|a, b| a / b),
                OpCode::Unknown => panic!(),
            }
        }
        InterpretResult::RuntimeError
    }

    fn binary_op<T>(&mut self, op: T)
    where
        T: FnOnce(f64, f64) -> f64,
    {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(op(a, b));
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}
