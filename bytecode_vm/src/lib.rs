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

#[derive(Debug)]
pub struct VirtualMachine {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: Vec<Value>,
}

impl VirtualMachine {
    pub fn init_machine(chunk: Chunk) -> VirtualMachine {
        VirtualMachine {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) {
        self.chunk = chunk;
        self.ip = 0;
        self.run();
    }

    pub fn run(&mut self) {
        
    }
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

    pub fn disassemble(&self, name: &str){
        println!("{} = {:?}", name, self.code);
        // for i in 0..self.code.len(){
        //     println!("{:?}", self.code);
        // }
    }


    pub fn disassemble_instruction(&mut self, offset: usize){
        println!("{:?}", self.code.get(offset..));
    }

    pub fn add_constant(&mut self, num: u8) -> u8 {

        // push opconstant into code, then push num into the next value slot then take the index of the value and push it into code.

        self.code.push(1);
        self.values.push(num);
        let temp_num: u8 = self.values.len() as u8;
        self.code.push(temp_num);
        temp_num
        
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