pub mod scanner;
pub mod compiler; 
use crate::compiler::Compiler;
use scanner::{Scanner, TokenType};


//pub type Value = i16;
pub type Number = i16;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Value{
    ValBool(bool),
    ValNumber(Number),
    ValNil
}

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
    OpNil,
    OpTrue,
    OpFalse,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess
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
            OpCode::OpNil      => 8,
            OpCode::OpTrue     => 9,
            OpCode::OpFalse    => 10,
            OpCode::OpNot      => 11,
            OpCode::OpEqual    => 12,
            OpCode::OpGreater  => 13,
            OpCode::OpLess      => 14
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
            8 => OpCode::OpNil,
            9 => OpCode::OpTrue,
            10 => OpCode::OpFalse,
            11 => OpCode::OpNot,
            12 => OpCode::OpEqual,
            13 => OpCode::OpGreater,
            14 => OpCode::OpLess,
            _ => unreachable!(),
        }
    }
}

// --- Chunk -----------------
#[derive(Debug, Default, Clone)]
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

    pub fn add_constant(&mut self, num: Value) -> u8 {
      self.values.push(num);
      (self.values.len() - 1) as u8
  }

    // Pretty disassembler you wrote
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset: usize = 0;
        while offset < self.code.len() {
            let val = self.disassemble_instruction(offset);
            offset += val;
            
           
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize{
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
                    let offset = 2; // opcode + operand
                    offset
                }
                OpCode::OpReturn => {
                    println!("OP_RETURN");
                    let offset = 1;
                    offset
                }
                OpCode::OpNegate => {
                    println!("OP_NEGATE");
                    let offset = 1;
                    offset
                }
                OpCode::OpAdd => {
                    println!("OP_ADD");
                    let offset = 1;
                    offset
                }
                OpCode::OpSubtract => {
                    println!("OP_SUBTRACT");
                    let offset = 1;
                    offset
                }
                OpCode::OpMultiply => {
                    println!("OP_MULTIPLY");
                    let offset = 1;
                    offset
                }
                OpCode::OpDivide => {
                    println!("OP_DIVIDE");
                    let offset = 1;
                    offset
                }
                OpCode::OpModulo => {
                    println!("OP_MODULO");
                    let offset = 1;
                    offset
                }
                OpCode::OpNil =>{
                    println!("OP_NIL");
                    let offset = 1;
                    offset
                }
                OpCode::OpTrue =>{
                    println!("OP_TRUE");
                    let offset = 1;
                    offset
                }
                OpCode::OpFalse =>{
                    println!("OP_FALSE");
                    let offset = 1;
                    offset
                }
                OpCode::OpNot =>{
                    println!("OP_NOT");
                    let offset = 1;
                    offset
                }
                OpCode::OpEqual=>{
                    println!("OP_EQUAL");
                    let offset = 1;
                    offset
                }
                OpCode::OpGreater=>{
                    println!("OP_GREATER");
                    let offset = 1;
                    offset
                }
                OpCode::OpLess=>{
                    println!("OP_LESS");
                    let offset = 1;
                    offset
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

    fn values_equal(a: Value, b: Value) -> bool {
    match (a, b) {
        (Value::ValBool(x), Value::ValBool(y)) => x == y,
        (Value::ValNumber(x), Value::ValNumber(y)) => x == y,
        (Value::ValNil, Value::ValNil) => true,
        _ => false,
            }
    }
    fn is_falsey(val: &Value) -> bool {
            match val {
                Value::ValNil => true,
                Value::ValBool(false) => true,
                _ => false,
            }
        }
     fn runtime_error( self: &mut VirtualMachine, message: &str ) {
            println!("{}", message );
            println!("[line {}] in script", self.chunk.lines[self.ip]);
        }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }
    pub fn interpret_source(&mut self, source_code: &str) -> InterpretResult {
        // Use the real compiler to produce bytecode, then run it
        let mut the_compiler = Compiler::init_compiler();
        if !the_compiler.compile(source_code) {
            println!("Finished Compiling");
            return InterpretResult::InterpretCompileError;
        }
        println!("Starting run");
        self.chunk = the_compiler.get_chunk();
        self.ip = 0;
        self.run()
    }

    pub fn compile(&mut self, source_code: &str) {
        let mut scanner: Scanner = Scanner::init_scanner(source_code);
        let mut line: usize = 0;

        loop {
            let token = scanner.scan_token();

            if token.line != line {
                print!("{:4} ", token.line);
                line = token.line;
            } else {
                print!("   | ");
            }

            println!(
                "{:?} {}, {:?}",
                token.token_type,
                token.length,
                String::from_utf8(token.value.clone())
            );

            if let TokenType::TokenEof = token.token_type {
                break;
            }
        }
    }

    pub fn run(&mut self) -> InterpretResult {

      while self.ip < self.chunk.code.len() {

          let byte = self.chunk.code[self.ip];
          let opcode = OpCode::BitToOp(byte);
          //self.chunk.disassemble_instruction(self.ip);
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
                    //   let neg = (0u8).wrapping_sub(v);
                    //   self.stack.push(neg);
                    self.stack.push(v * -1);
                    self.ip += 1;
                  } else {
                      return InterpretResult::InterpretRuntimeError;
                  }
              }
  
              OpCode::OpAdd => {
                  if let (Some(b), Some((a))) = (self.stack.pop(), self.stack.pop()) {
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

              OpCode::OpNegate =>{
                match self.stack.pop(){
                    Some(Value::ValNumber(n)) => self.stack.push(Value::ValNumber((-n))),
                    Some(_) => {
                        self.runtime_error("Operand must be a number");
                        return InterpretResult::InterpretRuntimeError
                    }
                    None => {
                        self.runtime_error("Operand must be a number");
                        return InterpretResult::InterpretRuntimeError
                    }
                }
              }

              OpCode::OpNil=>{
                self.stack.push(Value::ValNil)
              }

              OpCode::OpFalse=>{
                self.stack.push(Value::ValBool(false))
              }
              OpCode::OpTrue=>{
                self.stack.push(Value::ValBool(true))
              }
              OpCode::OpNot => {
                  match self.stack.pop() {
                      Some(v) => {
                          let result = !VirtualMachine::is_falsey(&v);
                          self.stack.push(Value::ValBool(result));
                          self.ip += 1;
                      }
                      None => {
                          return InterpretResult::InterpretRuntimeError;
                      }
                  }
              }
             OpCode::OpEqual =>{
                self.stack.push(OpCode::OpEqual);
             } 
             OpCode::OpGreater =>{

             }
             OpCode::OpEqual=> {

             }
        }
          
          
      }
  
      InterpretResult::InterpretSuccess
    }
}


 

#[cfg(test)]
mod tests {
    use super::*;

    // ---------Helpers ----------
    fn push_const(chunk: &mut Chunk, line: u32, v: Value) {
        let idx = chunk.add_constant(v);
        chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpConstant), line);
        chunk.write_to_chunk(idx, line);
    }

    fn push_op(chunk: &mut Chunk, line: u32, op: OpCode) {
        chunk.write_to_chunk(OpCode::OpToBit(op), line);
    }

    fn run(chunk: Chunk) -> (InterpretResult, Vec<Value>) {
        let mut vm = VirtualMachine::init_machine();
        let res = vm.interpret(chunk);
        (res, vm.stack)
    }

    // -----Opcode table---------
    #[test]
    fn opcode_roundtrip() {
        let ops = [
            OpCode::OpReturn,
            OpCode::OpConstant,
            OpCode::OpNegate,
            OpCode::OpAdd,
            OpCode::OpSubtract,
            OpCode::OpMultiply,
            OpCode::OpDivide,
            OpCode::OpModulo,
        ];
        for op in ops {
            let b = OpCode::OpToBit(op);
            let round = OpCode::BitToOp(b);
            assert_eq!(op, round, "Roundtrip failed for {:?}", op);
        }
    }

    // ------- Structure-----
    #[test]
    fn line_tracking_matches_code_bytes() {
        let mut c = Chunk::init_chunk();
        //each write_to_chunk must push a matching line number
        push_const(&mut c, 100, 1);
        push_const(&mut c, 101, 2);
        push_op(&mut c, 102, OpCode::OpAdd);
        push_op(&mut c, 103, OpCode::OpReturn);

        assert_eq!(c.code.len(), c.lines.len(), "code/line length mismatch");
        assert_eq!(c.values, vec![1, 2]);
        assert_eq!(c.lines[0], 100);
        assert_eq!(c.lines[2], 101);
        assert_eq!(c.lines[4], 102);
        assert_eq!(c.lines[5], 103);
    }

    #[test]
    fn disassemble_instruction_offsets() {
        let mut c = Chunk::init_chunk();
        push_const(&mut c, 10, 42);    // 2 bytes
        push_op(&mut c, 11, OpCode::OpNegate); // 1 byte
        push_op(&mut c, 12, OpCode::OpReturn); // 1 byte

        //xxpect: at 0 -> +2, at 2 -> +1, at 3 -> +1
        let mut off = 0;
        off += c.disassemble_instruction(off);
        assert_eq!(off, 2);
        off += c.disassemble_instruction(off);
        assert_eq!(off, 3);
        off += c.disassemble_instruction(off);
        assert_eq!(off, 4);
        assert_eq!(off, c.code.len());
    }

    // ----------arithmetic ----------
    #[test]
    fn constant_and_return_stack_top() {
        let mut c = Chunk::init_chunk();
        push_const(&mut c, 1, Value::ValNumber(123));
        push_op(&mut c, 2, OpCode::OpReturn);

        let (res, stack) = run(c);
        assert!(matches!(res, InterpretResult::InterpretSuccess));
        assert_eq!(stack, vec![123]);
    }

    #[test]
    fn add_sub_mul_div_mod_chain() {
        //program mirrors the sequence from the user’s main:
        // 42 18 + 32 / 37 * 85 % 12 negate; return
        let mut c = Chunk::init_chunk();

        push_const(&mut c, 123, 42);
        push_const(&mut c, 124, 18);
        push_op(&mut c, 125, OpCode::OpAdd);

        push_const(&mut c, 126, 32);
        push_op(&mut c, 127, OpCode::OpDivide);

        push_const(&mut c, 128, 37);
        push_op(&mut c, 129, OpCode::OpMultiply);

        push_const(&mut c, 130, 85);
        push_op(&mut c, 131, OpCode::OpModulo);

        push_const(&mut c, 132, 12);
        push_op(&mut c, 133, OpCode::OpNegate);

        push_op(&mut c, 134, OpCode::OpReturn);

        let (res, stack) = run(c);
        assert!(matches!(res, InterpretResult::InterpretSuccess));

        //compute expected with i16 math (no wrapping on / and % in code):
        // (((42 + 18) / 32) * 37) % 85 = ((60 / 32) * 37) % 85 = (1 * 37) % 85 = 37
        //push 12; negate => -12
        // final stack should be [37, -12]
        assert_eq!(stack, vec![37, -12]);
    }

    #[test]
    fn negate_positive_and_negative() {
        let mut c = Chunk::init_chunk();
        push_const(&mut c, 10, 7);
        push_op(&mut c, 11, OpCode::OpNegate); // -> -7
        push_const(&mut c, 12, -3);
        push_op(&mut c, 13, OpCode::OpNegate); // -> +3
        push_op(&mut c, 14, OpCode::OpReturn);

        let (res, stack) = run(c);
        assert!(matches!(res, InterpretResult::InterpretSuccess));
        assert_eq!(stack, vec![-7, 3]);
    }

    #[test]
    fn wrapping_add_sub_mul() {
        // uses i16::wrapping_* semantics
        let mut c = Chunk::init_chunk();
        push_const(&mut c, 1, i16::MAX); // 32767
        push_const(&mut c, 2, 1);
        push_op(&mut c, 3, OpCode::OpAdd); // wraps to -32768

        push_const(&mut c, 4, i16::MIN); // -32768
        push_const(&mut c, 5, 1);
        push_op(&mut c, 6, OpCode::OpSubtract); // (-32768) - 1 => wraps to 32767

        push_const(&mut c, 7, 2000);
        push_const(&mut c, 8, 20);
        push_op(&mut c, 9, OpCode::OpMultiply); // 2000*20 = 40000 -> wraps to i16

        push_op(&mut c, 10, OpCode::OpReturn);

        let (res, stack) = run(c);
        assert!(matches!(res, InterpretResult::InterpretSuccess));

        //compute expected wrapping
        let a = i16::MAX.wrapping_add(1);          // -32768
        let b = i16::MIN.wrapping_sub(1);          // 32767
        let cval = 2000i16.wrapping_mul(20);       // wrapping product

        assert_eq!(stack, vec![a, b, cval]);
    }

    // ---------- error handling ----------
    #[test]
    fn error_missing_constant_operand() {
        // OpConstant with no following index byte
        let mut c = Chunk::init_chunk();
        push_op(&mut c, 1, OpCode::OpConstant);
        // no operand byte
        let (res, _stack) = run(c);
        assert!(matches!(res, InterpretResult::InterpretRuntimeError));
    }

    #[test]
    fn error_bad_constant_index() {
        // OpConstant with index that is out of bounds
        let mut c = Chunk::init_chunk();
        c.write_to_chunk(OpCode::OpToBit(OpCode::OpConstant), 1);
        c.write_to_chunk(99, 1); // bogus index; values.len() == 0
        let (res, _stack) = run(c);
        assert!(matches!(res, InterpretResult::InterpretRuntimeError));
    }

    #[test]
    fn error_stack_underflow_binary_ops() {
        //try each binary op with insufficient stack
        for op in [
            OpCode::OpAdd,
            OpCode::OpSubtract,
            OpCode::OpMultiply,
            OpCode::OpDivide,
            OpCode::OpModulo,
        ] {
            let mut c = Chunk::init_chunk();
            //push only one constant, then a binary op
            push_const(&mut c, 10, 1);
            push_op(&mut c, 11, op);
            let (res, _stack) = run(c);
            assert!(
                matches!(res, InterpretResult::InterpretRuntimeError),
                "Expected runtime error for {:?} with one operand",
                op
            );
        }
    }

    #[test]
    fn error_divide_by_zero() {
        let mut c = Chunk::init_chunk();
        push_const(&mut c, 1, 10);
        push_const(&mut c, 2, 0);
        push_op(&mut c, 3, OpCode::OpDivide);
        let (res, _stack) = run(c);
        assert!(matches!(res, InterpretResult::InterpretRuntimeError));
    }

    #[test]
    fn error_mod_by_zero() {
        let mut c = Chunk::init_chunk();
        push_const(&mut c, 1, 10);
        push_const(&mut c, 2, 0);
        push_op(&mut c, 3, OpCode::OpModulo);
        let (res, _stack) = run(c);
        assert!(matches!(res, InterpretResult::InterpretRuntimeError));
    }

    #[test]
    fn return_terminates_execution() {
        let mut c = Chunk::init_chunk();
        push_const(&mut c, 1, 5);
        push_op(&mut c, 2, OpCode::OpReturn);
        // garbage after return (should never execute)
        push_const(&mut c, 999, 12345);
        push_op(&mut c, 999, OpCode::OpAdd);

        let (res, stack) = run(c);
        assert!(matches!(res, InterpretResult::InterpretSuccess));
        assert_eq!(stack, vec![5], "VM should stop executing after OpReturn");
    }

    // ---------- order of operation----------
    #[test]
    fn left_to_right_eval_for_binary_ops() {
        // ((8 - 3) * 2) + (20 / 5) % 6 = (5 * 2) + (4) % 6 = 10 + 4 = 14
        let mut c = Chunk::init_chunk();
        push_const(&mut c, 1, 8);
        push_const(&mut c, 2, 3);
        push_op(&mut c, 3, OpCode::OpSubtract);  // 5

        push_const(&mut c, 4, 2);
        push_op(&mut c, 5, OpCode::OpMultiply);  // 10

        push_const(&mut c, 6, 20);
        push_const(&mut c, 7, 5);
        push_op(&mut c, 8, OpCode::OpDivide);    // 4

        push_const(&mut c, 9, 6);
        push_op(&mut c, 10, OpCode::OpModulo);   // 4 % 6 = 4

        push_op(&mut c, 11, OpCode::OpAdd);      // 10 + 4 = 14
        push_op(&mut c, 12, OpCode::OpReturn);

        let (res, stack) = run(c);
        assert!(matches!(res, InterpretResult::InterpretSuccess));
        assert_eq!(stack, vec![14]);
    }

// ----------------------- SCANNER TESTS ----------------------------

    fn collect_tokens(src: &str) -> Vec<TokenType> {
        let mut s = Scanner::init_scanner(src);
        let mut out = Vec::new();
        loop {
            let t = s.scan_token();
            out.push(t.token_type);
            if matches!(t.token_type, TokenType::TokenEof) { break; }
        }
        out
    }

    #[test]
    fn scan_single_char_tokens() {
        let src = "(){};,.-+*/";
        let got = collect_tokens(src);
        use TokenType::*;
        let expect = vec![
            TokenLeftParen, TokenRightParen,
            TokenLeftBrace, TokenRightBrace,
            TokenSemicolon,
            TokenComma,
            TokenDot,
            TokenMinus, TokenPlus,
            TokenStar, // careful: '/' is TokenSlash; '*' before '/'
            TokenSlash,
            TokenEof,
        ];
        assert_eq!(got, expect);
    }

    #[test]
    fn scan_two_char_comparisons() {
        let src = "! != = == < <= > >=";
        // include spaces to exercise whitespace skipper
        let got = collect_tokens(src);
        use TokenType::*;
        let expect = vec![
            TokenNot, TokenNotEqual, TokenEqual, TokenEqualEqual,
            TokenLess, TokenLessEqual, TokenGreater, TokenGreaterEqual,
            TokenEof,
        ];
        assert_eq!(got, expect);
    }

    #[test]
    fn scan_numbers_and_identifiers() {
        let src = "var x = 12.34\ny_2 = 7\n";
        let mut s = Scanner::init_scanner(src);

        let mut next = || s.scan_token().token_type;

        use TokenType::*;
        assert_eq!(next(), TokenVar);
        assert_eq!(next(), TokenIdentifier); // x
        assert_eq!(next(), TokenEqual);
        assert_eq!(next(), TokenNumber);     // 12.34
        assert_eq!(next(), TokenIdentifier); // y_2 (identifier allows digits after first)
        assert_eq!(next(), TokenEqual);
        assert_eq!(next(), TokenNumber);     // 7
        assert_eq!(next(), TokenEof);
    }

    #[test]
    fn scan_strings_and_comments() {
        let src = r#"
            // this is a comment
            print "hello";
            // another comment
        "#;

        let got = collect_tokens(src);
        use TokenType::*;
        let expect = vec![
            TokenPrint, TokenString, TokenSemicolon,
            TokenEof,
        ];
        assert_eq!(got, expect);
    }

    #[test]
    fn unterminated_string_errors() {
        let src = "print \"oops";
        let mut s = Scanner::init_scanner(src);

        let _ = s.scan_token(); // print
        let tok = s.scan_token(); // should be error
        assert!(matches!(tok.token_type, TokenType::TokenError));
        let msg = String::from_utf8(tok.value).unwrap();
        assert!(msg.contains("Unterminated"), "got error: {}", msg);
    }

    #[test]
    fn keyword_recognition() {
        let src = "and class else false for fun if nil or print return super this true var while";
        let got = collect_tokens(src);
        use TokenType::*;
        let expect = vec![
            TokenAnd, TokenClass, TokenElse, TokenFalse, TokenFor, TokenFun,
            TokenIf, TokenNil, TokenOr, TokenPrint, TokenReturn, TokenSuper,
            TokenThis, TokenTrue, TokenVar, TokenWhile, TokenEof,
        ];
        assert_eq!(got, expect);
    }

    #[test]
    fn line_number_progresses_but_tokens_only_on_code_lines() {
        let src = "var x = 1;\nprint x;\n\n// comment\nx = 2;";
        let mut s = Scanner::init_scanner(src);

        let mut toks = Vec::new();
        loop {
            let t = s.scan_token();
            toks.push(t);
            if matches!(toks.last().unwrap().token_type, TokenType::TokenEof) { break; }
        }

        use std::collections::BTreeSet;
        let lines: BTreeSet<usize> = toks.iter().map(|t| t.line).collect();

        // Tokens appear on code lines 1, 2, and 5
        assert!(lines.contains(&1));
        assert!(lines.contains(&2));
        assert!(lines.contains(&5));

        // No tokens should be produced for the blank line (3) or comment-only line (4)
        assert!(!lines.contains(&3));
        assert!(!lines.contains(&4));
    }


    #[test]
    fn unknown_character_produces_error() {
        let src = "@";
        let mut s = Scanner::init_scanner(src);
        let t = s.scan_token();
        assert!(matches!(t.token_type, TokenType::TokenError));
       let msg = String::from_utf8(t.value).unwrap();
        assert!(msg.contains("Unknown"));
    }
}



 
