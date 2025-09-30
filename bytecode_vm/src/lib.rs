
pub type Value = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpModulo,
}

impl OpCode {
    pub fn OpToBit(name: OpCode) -> u8 {
        match name {
            OpCode::OpReturn   => 0,
            OpCode::OpConstant => 1,
            OpCode::OpNegate   => 2,
            OpCode::OpAdd      => 3,
            OpCode::OpSubtract => 4,
            OpCode::OpMultiply => 5,
            OpCode::OpDivide   => 6,
            OpCode::OpModulo   => 7,
        }
    }

    pub fn BitToOp(num: u8) -> OpCode {
        match num {
            0 => OpCode::OpReturn,
            1 => OpCode::OpConstant,
            2 => OpCode::OpNegate,
            3 => OpCode::OpAdd,
            4 => OpCode::OpSubtract,
            5 => OpCode::OpMultiply,
            6 => OpCode::OpDivide,
            7 => OpCode::OpModulo,
            _ => unreachable!(),
        }
    }
}

// --- Chunk -------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Chunk {
    pub code: Vec<u8>,   // opcode bytes (and any inline operands)
    pub lines: Vec<u32>, // line per byte in code
    pub values: Vec<Value>, // constant pool
}

impl Chunk {
    pub fn init_chunk() -> Chunk {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn write_to_chunk(&mut self, byte: u8, linenum: u32) {
        self.code.push(byte);
        self.lines.push(linenum);
    }

    pub fn add_constant(&mut self, num: u8) -> u8 {
      self.values.push(num);
      (self.values.len() - 1) as u8
  }

    // Pretty disassembler you wrote
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset: usize = 0;
        while offset < self.code.len() {
            // byte offset
            print!("{:04}  ", offset);

            // line (assumes 1 line per code byte push)
            let line = self.lines[offset];
            print!("{:>3} ", line);

            // decode
            let opcodebyte = self.code[offset];
            let opcode = OpCode::BitToOp(opcodebyte);

            match opcode {
                OpCode::OpConstant => {
                    // one-byte operand: constant index
                    let idxbyte = self.code[offset + 1] as usize;
                    let value = self.values[idxbyte];
                    println!("OP_CONSTANT         {} {}", idxbyte, value);
                    offset += 2; // opcode + operand
                }
                OpCode::OpReturn => {
                    println!("OP_RETURN");
                    offset += 1;
                }
                OpCode::OpNegate => {
                    println!("OP_NEGATE");
                    offset += 1;
                }
                OpCode::OpAdd => {
                    println!("OP_ADD");
                    offset += 1;
                }
                OpCode::OpSubtract => {
                    println!("OP_SUBTRACT");
                    offset += 1;
                }
                OpCode::OpMultiply => {
                    println!("OP_MULTIPLY");
                    offset += 1;
                }
                OpCode::OpDivide => {
                    println!("OP_DIVIDE");
                    offset += 1;
                }
                OpCode::OpModulo => {
                    println!("OP_MODULO");
                    offset += 1;
                }
            }
        }
    }

    pub fn disassemble_instruction(&self, mut offset: usize) {
        //println!("{:?}", self.code.get(offset..));
        
            // byte offset
            print!("{:04}  ", offset);

            // line (assumes 1 line per code byte push)
            let line = self.lines[offset];
            print!("{:>3} ", line);

            // decode
            let opcodebyte = self.code[offset];
            let opcode = OpCode::BitToOp(opcodebyte);

            match opcode {
                OpCode::OpConstant => {
                    // one-byte operand: constant index
                    let idxbyte = self.code[offset + 1] as usize;
                    let value = self.values[idxbyte];
                    println!("OP_CONSTANT         {} {}", idxbyte, value);
                    offset += 2; // opcode + operand
                }
                OpCode::OpReturn => {
                    println!("OP_RETURN");
                    offset += 1;
                }
                OpCode::OpNegate => {
                    println!("OP_NEGATE");
                    offset += 1;
                }
                OpCode::OpAdd => {
                    println!("OP_ADD");
                    offset += 1;
                }
                OpCode::OpSubtract => {
                    println!("OP_SUBTRACT");
                    offset += 1;
                }
                OpCode::OpMultiply => {
                    println!("OP_MULTIPLY");
                    offset += 1;
                }
                OpCode::OpDivide => {
                    println!("OP_DIVIDE");
                    offset += 1;
                }
                OpCode::OpModulo => {
                    println!("OP_MODULO");
                    offset += 1;
                }
            }
        }
    }


// --- VM ----------------------------------------------------------------------

pub enum InterpretResult {
    InterpretSuccess,
    InterpretCompileError,
    InterpretRuntimeError,
}

#[derive(Debug)]
pub struct VirtualMachine {
    pub chunk: Chunk,
    pub ip: usize,     
    pub stack: Vec<Value>,
}

impl VirtualMachine {
    pub fn init_machine() -> VirtualMachine {
        VirtualMachine {
            chunk: Chunk::init_chunk(),
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    pub fn run(&mut self) -> InterpretResult {

      while self.ip < self.chunk.code.len() {

          let byte = self.chunk.code[self.ip];
          let opcode = OpCode::BitToOp(byte);
          self.chunk.disassemble_instruction(self.ip);
          println!("{:?}", self.stack);
          
          match opcode {
              OpCode::OpReturn => {
                  self.ip += 1;
                  return InterpretResult::InterpretSuccess;
              }
  
              OpCode::OpConstant => {
                  if self.ip + 1 >= self.chunk.code.len() {
                      return InterpretResult::InterpretRuntimeError;
                  }
                  let idx = self.chunk.code[self.ip + 1] as usize;
                  if idx >= self.chunk.values.len() {
                      return InterpretResult::InterpretRuntimeError;
                  }
                  let val = self.chunk.values[idx];
                  self.stack.push(val);
                  self.ip += 2;
              }
  
              OpCode::OpNegate => {
                  if let Some(v) = self.stack.pop() {
                      let neg = (0u8).wrapping_sub(v);
                      self.stack.push(neg);
                      self.ip += 1;
                  } else {
                      return InterpretResult::InterpretRuntimeError;
                  }
              }
  
              OpCode::OpAdd => {
                  if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                      self.stack.push(a.wrapping_add(b));
                      self.ip += 1;
                  } else {
                      return InterpretResult::InterpretRuntimeError;
                  }
              }
  
              OpCode::OpSubtract => {
                  if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                      self.stack.push(a.wrapping_sub(b));
                      self.ip += 1;
                  } else {
                      return InterpretResult::InterpretRuntimeError;
                  }
              }
  
              OpCode::OpMultiply => {
                  if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                      self.stack.push(a.wrapping_mul(b));
                      self.ip += 1;
                  } else {
                      return InterpretResult::InterpretRuntimeError;
                  }
              }
  
              OpCode::OpDivide => {
                  if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                      if b == 0 {
                          return InterpretResult::InterpretRuntimeError;
                      }
                      self.stack.push(a / b);
                      self.ip += 1;
                  } else {
                      return InterpretResult::InterpretRuntimeError;
                  }
              }
  
              OpCode::OpModulo => {
                  if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                      if b == 0 {
                          return InterpretResult::InterpretRuntimeError;
                      }
                      self.stack.push(a % b);
                      self.ip += 1;
                  } else {
                      return InterpretResult::InterpretRuntimeError;
                  }
              }
          }
          
      }
  
      InterpretResult::InterpretSuccess
    }
}
 
