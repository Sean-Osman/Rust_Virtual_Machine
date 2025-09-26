pub type Value = u8;

#[derive(Debug)]
pub enum OpCode{
    OpReturn,
    OpConstant,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,

}

#[derive(Debug, Default)]
pub struct Chunk{
    pub code: Vec<u8>, //what opcode
    pub lines: Vec<u32>, //specific line location
    pub values: Vec<Value> // the actual number

}

impl Chunk{
    //changed to no self param
    pub fn init_chunk() -> Chunk{
      return Chunk { code:Vec::new(),lines:Vec::new(),values:Vec::new() }
  }

    pub fn write_to_chunk(&mut self, byte:u8, linenum:u32){
        self.code.push(byte);
        self.lines.push(linenum);
    }

    // pub fn disassemble(&self, name: &str){
    //     println!("{} = {:?}", name, self.code);
    //     // for i in 0..self.code.len(){
    //     //     println!("{:?}", self.code);
    //     // }
    // }
    pub fn disassemble(&self, name: &str) {
      println!("== {} ==", name);
  
      let mut offset: usize = 0;
  
      while offset < self.code.len() {
          print!("{:04}  ", offset);
          let line = self.lines[offset];
          print!("{:>3} ", line);
  
          
          let opcodebyte = self.code[offset];
          let opcode = OpCode::BitToOp(opcodebyte);
  
          match opcode {
              OpCode::OpConstant => {
                  let idxbyte = self.code[offset + 1] as usize;
                  let value = self.values[idxbyte];
                  println!("OP_CONSTANT         {} {}", idxbyte, value);
                  offset += 2;
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
          }
      }
  }
    


    pub fn disassemble_instruction(&mut self, offset: usize){
        println!("{:?}", self.code.get(offset..));
    }

    pub fn add_constant(&mut self, num: u8) -> u8 {
      // push opconstant into code, then push num into the next value slot then take the index of the value and push it into code.
      //needs further implementation but for now works.
      self.values.push(num);
      (self.values.len() - 1) as u8;
      
    }
  
  
}
 impl OpCode{


    pub fn OpToBit(name: OpCode) -> u8{

        match(name){
            OpCode::OpReturn => 0,
            OpCode::OpConstant => 1,
            OpCode::OpNegate => 2,
            OpCode::OpAdd => 3,
            OpCode::OpSubtract => 4,
            OpCode::OpMultiply => 5,
            OpCode::OpDivide => 6
        }
    }

    pub fn BitToOp(num: u8) -> OpCode{

        match(num){
            0 => OpCode::OpReturn,
            1 => OpCode::OpConstant,
            2 => OpCode::OpNegate,
            3 => OpCode::OpAdd,
            4 => OpCode::OpSubtract,
            5 => OpCode::OpMultiply,
            6 => OpCode::OpDivide,
            _ => unreachable!()
        }

    }
}
