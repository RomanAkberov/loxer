pub type Value = f64;

#[derive(Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<u32>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        let index = self.constants.len();
        self.constants.push(value);
        index
    }
}

pub fn disassemble(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0;
    while offset < chunk.code.len() {
        print!("{:04} ", offset);
        if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", chunk.lines[offset]);
        }
        let instruction = chunk.code[offset];
        offset += match instruction {
            op::RETURN => simple_instruction("OP_RETURN"),
            op::CONSTANT => constant_instruction("OP_CONSTANT", chunk, offset),
            op::NEGATE => simple_instruction("OP_NEGATE"),
            op::ADD => simple_instruction("OP_ADD"),
            op::SUBTRACT => simple_instruction("OP_SUBTRACT"),
            op::MULTIPLY => simple_instruction("OP_MULTIPLY"),
            op::DIVIDE => simple_instruction("OP_DIVIDE"),
            _ => panic!("Illegal instruction {}", instruction),
        }
    }
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{} {:4} '", name, constant);
    print_value(chunk.constants[constant as usize]);
    println!("'");
    2
}

fn print_value(value: f64) {
    print!("{}", value);
}

fn simple_instruction(name: &str) -> usize {
    println!("{}", name);
    1
}

pub mod op {
    pub const CONSTANT: u8 = 0;
    pub const RETURN: u8 = 1;
    pub const NEGATE: u8 = 2;
    pub const ADD: u8 = 3;
    pub const SUBTRACT: u8 = 4;
    pub const MULTIPLY: u8 = 5;
    pub const DIVIDE: u8 = 6;
}

const STACK_SIZE: usize = 256;

pub struct VirtualMachine {
    ip: usize,
    stack: [Value; STACK_SIZE],
    stack_top: usize,
}

impl VirtualMachine {
    pub fn run(&mut self, chunk: &Chunk) -> Result<(), Error> {
        self.ip = 0;
        loop {
            let instruction = self.read_byte(chunk);
            match instruction {
                op::RETURN => {
                    println!("{}", self.pop());
                    return Ok(());
                }
                op::CONSTANT => {
                    let value = self.read_constant(chunk);
                    self.push(value);
                }
                op::NEGATE => self.unary(|a| -a),
                op::ADD => self.binary(|a, b| a + b),
                op::SUBTRACT => self.binary(|a, b| a - b),
                op::MULTIPLY => self.binary(|a, b| a * b),
                op::DIVIDE => self.binary(|a, b| a / b),
                _ => {}
            }
        }
    }

    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let constant = self.read_byte(chunk);
        chunk.constants[constant as usize]
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let byte = chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    fn binary<F>(&mut self, op: F)
    where
        F: Fn(Value, Value) -> Value,
    {
        let left = self.pop();
        let right = self.pop();
        self.push(op(left, right));
    }

    fn unary<F>(&mut self, op: F)
    where
        F: Fn(Value) -> Value,
    {
        let arg = self.pop();
        self.push(op(arg));
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self {
            ip: Default::default(),
            stack: [Value::default(); STACK_SIZE],
            stack_top: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn disassemble_something() {
        let mut chunk = Chunk::default();
        let c0 = chunk.add_constant(1.2);
        let c1 = chunk.add_constant(-9.3);
        chunk.write(op::CONSTANT, 123);
        chunk.write(c0 as u8, 123);
        chunk.write(op::CONSTANT, 123);
        chunk.write(c1 as u8, 123);
        chunk.write(op::ADD, 123);
        chunk.write(op::RETURN, 123);
        disassemble(&chunk, "test chunk");
        let mut vm = VirtualMachine::default();
        println!("{:?}", vm.run(&chunk));
    }
}
